use crate::{
  communication::{RecvMessage, SendableMessage},
  consts, ArmLevel, ArmMode, ClientError, Keypress, ListRequest,
};
use futures::{SinkExt, Stream, StreamExt};
use std::{
  future::Future,
  io,
  pin::Pin,
  str,
  task::{Context, Poll},
  time::{Duration, Instant},
};
use tokio::sync::mpsc;
use tokio_serial::{SerialPort, SerialPortBuilderExt, SerialStream};
use tokio_util::{
  bytes::{Buf, BufMut, BytesMut},
  codec::{Decoder, Encoder, Framed},
};

const BYTE: usize = consts::ASCII_BYTE_REAL_LEN;
const TIMEOUT_THRESHOLD: Duration = Duration::from_secs(2);

#[derive(Debug)]
pub struct Serial {
  has_errored: bool,
  preparing: bool,
  ready: bool,

  sending: bool,
  resend: bool,
  last_send: Instant,
  retry_count: u8,
  last_message: Option<SendableMessage>,

  serial: Framed<SerialStream, Concord4Codec>,

  pub tx: mpsc::Sender<SendableMessage>,
  rx: mpsc::Receiver<SendableMessage>,
}

impl Serial {
  pub async fn init(path: &str) -> Result<Self, ClientError> {
    let port = tokio_serial::new(path, consts::BAUD_RATE)
      .data_bits(consts::DATA_BITS)
      .parity(consts::PARITY)
      .timeout(Duration::from_millis(10))
      .open_native_async()?;

    tracing::info!("Receiving data on {} at 9600 baud:", path);

    // must clear buffer because system won't know which message the ACKs are referring to
    port.clear(tokio_serial::ClearBuffer::All)?;

    let serial = Concord4Codec.framed(port);
    let (tx, rx) = mpsc::channel(32);

    Ok(Self {
      has_errored: false,
      preparing: false,
      ready: false,

      sending: false,
      resend: false,
      last_send: Instant::now(),
      retry_count: 0,
      last_message: None,

      serial,

      tx,
      rx,
    })
  }

  async fn serial_loop(&mut self) -> Result<Option<RecvMessage>, ClientError> {
    if !self.ready && !self.preparing && !self.sending && self.last_send.elapsed() > Duration::from_secs(10) {
      tracing::info!(target: "concord4::serial::loop", "serial port has been open for 10 seconds and a clear image has not been received, manually sending list request");

      if let Err(err) = self.tx.send(SendableMessage::List(ListRequest::AllData)).await {
        tracing::error!(target: "concord4::serial::loop", "failed to send list request: {:?}", err);
        return Err(ClientError::Sender(err));
      }

      self.preparing = true;
    }

    if self.resend {
      tracing::warn!(target: "concord4::serial::loop", "resending message because no response received; times retried: {}", self.retry_count + 1);

      if let Some(last_message) = &self.last_message {
        tracing::debug!(target: "concord4::serial::loop", "resending message: {:?}", last_message);

        if let Err(err) = self.serial.send(last_message.clone()).await {
          tracing::error!(target: "concord4::serial::loop", "failed to resend message: {:?}", err);

          return Err(ClientError::Encoder(err));
        } else {
          self.resend = false;
          self.last_send = Instant::now();
          self.sending = true;
          self.retry_count += 1;
        }
      }
    }

    tokio::select! {
      biased;
      result = self.serial.next() => {
        if let Some(result) = result {
          match result {
            Ok(RecvMessage::Ack) | Ok(RecvMessage::Nak) => {
              match result {
                Ok(RecvMessage::Ack) => tracing::debug!(target: "concord4::serial::loop", "received ACK"),
                Ok(RecvMessage::Nak) => tracing::warn!(target: "concord4::serial::loop", "received NAK"),
                _ => unreachable!(),
              };

              self.resend = false;
              self.sending = false;
              self.last_message = None;
              self.retry_count = 0;

              result.map(Some).map_err(ClientError::Decoder)
            }
            _ => {
              if let Err(err) = self.serial.send(SendableMessage::Ack).await {
                tracing::error!(target: "concord4::serial::loop", "failed to send ack: {:?}", err);
              }

              if let Ok(RecvMessage::EqptListDone) = result {
                if !self.ready {
                  tracing::info!(target: "concord4::serial::loop", "panel is ready to go!");
                  if let Err(err) = self.tx.send(SendableMessage::DynamicDataRefresh).await {
                    tracing::error!(target: "concord4::serial::loop", "failed to send dynamic data refresh: {:?}", err);
                  }

                  self.preparing = false;
                  self.ready = true;
                }
              }

              if let Ok(RecvMessage::ClearImage(_)) = result {
                tracing::info!(target: "concord4::serial::loop", "panel requested an image reset");
                self.preparing = true;

                if let Err(err) = self.tx.send(SendableMessage::List(ListRequest::AllData)).await {
                  tracing::error!(target: "concord4::serial::loop", "failed to send list request: {:?}", err);
                  return Err(ClientError::Sender(err));
                }
              }

              result.map(Some).map_err(ClientError::Decoder)
            },
          }
        } else {
          tracing::error!(target: "concord4::serial::loop", "serial port closed");
          self.has_errored = true;

          Err(ClientError::SerialPortClosed)
        }
      },
      Some(message) = self.rx.recv(), if {
        // do not receive messages until the panel is ready for them (post ack/nak)
        if self.sending && self.last_send.elapsed() < TIMEOUT_THRESHOLD {
          tracing::trace!(target: "concord4::serial::loop", "not ready to send message because waiting for response, waiting...");

          false
        } else if self.sending && self.last_send.elapsed() >= TIMEOUT_THRESHOLD {
          if self.retry_count < 5 {
            // resend the last message; we don't want to do this instantly though because the ack may be in flight
            // therefore we will mark the message for resend and wait for the next poll of the stream
            self.resend = true;
          } else {
            tracing::error!(target: "concord4::serial::loop", "maximum retries reached, giving up on message");
            self.sending = false;
            self.preparing = false;
            self.retry_count = 0;
          }

          false
        } else {
          true
        }
      } => {
        tracing::debug!(target: "concord4::serial::loop", "sending message: {:?}", message);
        if let Err(err) = self.serial.send(message.clone()).await {
          tracing::error!(target: "concord4::serial::loop", "failed to send message: {:?}", err);

          Err(ClientError::Encoder(err))
        } else {
          self.last_send = Instant::now();
          self.sending = true;
          self.last_message = Some(message);
          self.retry_count = 0;

          Ok(None)
        }
      },
      else => {
        tracing::error!(target: "concord4::serial::loop", "serial port closed");

        self.has_errored = true;
        Err(ClientError::SerialPortClosed)
      },
    }
  }
}

impl Stream for Serial {
  type Item = Result<RecvMessage, ClientError>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    if self.has_errored {
      return Poll::Ready(None);
    }

    let future = Pin::into_inner(self).serial_loop();
    futures::pin_mut!(future);

    match future.poll(cx) {
      Poll::Ready(Ok(Some(result))) => Poll::Ready(Some(Ok(result))),
      Poll::Ready(Ok(None)) => Poll::Pending,
      Poll::Ready(Err(err)) => Poll::Ready(Some(Err(err))),
      Poll::Pending => Poll::Pending,
    }
  }
}

#[derive(Debug)]
struct Concord4Codec;

impl Decoder for Concord4Codec {
  type Item = RecvMessage;
  type Error = io::Error;

  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    if src.is_empty() {
      return Ok(None);
    }

    tracing::trace!(target: "concord4::serial::decoder", "src: {:?}", src);

    let ctrl_ctr = src.as_ref().iter().position(|b| *b == consts::ACK || *b == consts::NAK);
    let newline = src.as_ref().iter().position(|b| *b == b'\n');

    if let Some(ctrl_pos) = ctrl_ctr {
      tracing::trace!(target: "concord4::serial::decoder","ctrl_ctr: {:?}", ctrl_ctr);
      // if there is a linefeed before the control character, we need to process that first
      if newline.is_some_and(|lf_pos| ctrl_pos < lf_pos) {
        let ctrl = *src.get(ctrl_pos).expect("infallible");

        let mut after_ctrl = src.split_off(ctrl_pos);
        after_ctrl.advance(1);
        src.extend(after_ctrl);

        if ctrl == consts::ACK {
          tracing::trace!(target: "concord4::serial::decoder","recv: ACK");
          return Ok(Some(RecvMessage::Ack));
        } else {
          tracing::warn!(target: "concord4::serial::decoder","recv: NAK");
          return Ok(Some(RecvMessage::Nak));
        }
      }
    }

    if let Some(lf_pos) = newline {
      // post_lf stands for position after line feed
      let post_lf = lf_pos + 1;

      // byte here stands for the byte denoting length of the message
      if src.len() < post_lf + BYTE {
        tracing::trace!(target: "concord4::serial::decoder","message not finished - waiting for more data");
        return Ok(None);
      }

      // byte here stands for the byte denoting length of the message since length doesn't include itself
      let (data_len, data_len_in_buffer) = if let Some(len_bytes) = src.get((post_lf)..(post_lf + BYTE)) {
        let data_len = ascii_hex_to_u8(len_bytes);
        // byte here stands for how each message in buffer is twice as long as the actual data
        let data_len_in_buffer = data_len as usize * BYTE;

        tracing::trace!(target: "concord4::serial::decoder","data_len: {} hex bytes; {} real bytes;", data_len, data_len_in_buffer);

        (data_len, data_len_in_buffer)
      } else {
        (0, 0)
      };

      // byte here stands for the byte denoting length of the message
      if data_len_in_buffer == 0 || src.len() < post_lf + BYTE + data_len_in_buffer {
        tracing::trace!(target: "concord4::serial::decoder","message not finished - waiting for more data");
        return Ok(None);
      }

      src.advance(post_lf);
      // byte here stands for the byte denoting length of the message since length doesn't include itself
      let full_data = src.split_to(BYTE + data_len_in_buffer);
      tracing::debug!(target: "concord4::serial::decoder","message received - {:?}", full_data);

      // byte here stands for the byte denoting length of the message
      let mut data = match ascii_hex_to_bin(full_data.get(BYTE..).expect("infallible")) {
        Ok(data) => data,
        Err(_) => return Ok(None),
      };
      let checksum = data.pop().expect("infallible");

      tracing::trace!(target: "concord4::serial::decoder","data: {:?}", data);
      tracing::trace!(target: "concord4::serial::decoder","checksum: {:?}", checksum);

      if validate_checksum(data_len, &data, checksum) {
        tracing::trace!(target: "concord4::serial::decoder","checksum valid");
      } else {
        tracing::error!(target: "concord4::serial::decoder","invalid checksum");

        return Ok(None);
      }

      let data = RecvMessage::try_from(data);
      if let Ok(data) = data {
        return Ok(Some(data));
      } else {
        tracing::error!(target: "concord4::serial::decoder","failed to parse message");
      }
    }

    Ok(None)
  }
}

impl Encoder<SendableMessage> for Concord4Codec {
  type Error = io::Error;

  fn encode(&mut self, item: SendableMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
    use tokio_util::bytes::BufMut;
    tracing::trace!(target: "concord4::serial::encoder","sending: {:?}", item);
    tracing::trace!(target: "concord4::serial::encoder","dst before: {:?}", dst);

    // first byte is length so zero for now
    let mut data: Vec<u8> = vec![0x0];

    match item {
      SendableMessage::Ack => {
        dst.put_u8(consts::ACK);

        return Ok(());
      }
      SendableMessage::Nak => {
        dst.put_u8(consts::NAK);

        return Ok(());
      }
      SendableMessage::List(req) => match req {
        ListRequest::AllData => {
          data.push(0x2);
        }
        other => {
          data.put_slice(&[0x2, other as u8]);
        }
      },
      SendableMessage::Arm(options) => {
        let mut keys = match options.mode {
          ArmMode::Stay => match options.level {
            Some(ArmLevel::Silent) => vec![Keypress::Five, Keypress::Two],
            _ => vec![Keypress::Two],
          },
          ArmMode::Away => match options.level {
            Some(ArmLevel::Silent) => vec![Keypress::Five, Keypress::Three],
            _ => vec![Keypress::Three],
          },
        };
        keys.extend_from_slice(&options.code);

        // instant arming is a keypress after the code
        if let Some(ArmLevel::Instant) = options.level {
          keys.push(Keypress::Four);
        }

        handle_keypress(&mut data, options.partition.unwrap_or(1), &keys);
      }
      SendableMessage::Disarm(options) => {
        let mut keys = [Keypress::One; 5];
        keys[1..].copy_from_slice(&options.code);

        handle_keypress(&mut data, options.partition.unwrap_or(1), &keys);
      }
      SendableMessage::ToggleChime(partition) => {
        handle_keypress(&mut data, partition.unwrap_or(1), &[Keypress::Seven, Keypress::One])
      }
      SendableMessage::Keypress(partition, keys) => {
        handle_keypress(&mut data, partition, &keys);
      }
      SendableMessage::DynamicDataRefresh => {
        data.push(0x20);
      }
    }

    // the length of the message doesn't include the length byte itself
    data[0] = data.len() as u8;
    data.push(compute_checksum(&data));
    let msg = ascii_hex_to_string(data.as_ref());
    let msg_with_lf = ["\n", &msg].concat();

    tracing::debug!(target: "concord4::serial::encoder","sending message: {:?}", msg_with_lf);
    tracing::trace!(target: "concord4::serial::encoder","sending message bytes: {:?}", msg_with_lf.as_bytes());

    dst.put(msg_with_lf.as_bytes());
    tracing::trace!(target: "concord4::serial::encoder","dst after: {:?}", dst);

    Ok(())
  }
}

fn handle_keypress(data: &mut Vec<u8>, partition: u8, keys: &[Keypress]) {
  data.put_slice(&[0x40, partition, 0x0]);
  data.extend(keys.iter().copied().map(Into::<u8>::into));
}

fn compute_checksum(msg: &[u8]) -> u8 {
  (msg.iter().map(|b| *b as usize).sum::<usize>() % 256) as u8
}

fn validate_checksum(first_byte: u8, msg: &[u8], checksum: u8) -> bool {
  let calc_sum = compute_checksum([&[first_byte], msg].concat().as_ref());

  checksum == calc_sum
}

fn ascii_hex_to_string(hex: &[u8]) -> String {
  use std::fmt::Write;
  hex.iter().fold(String::new(), |mut output, b| {
    let _ = write!(output, "{b:02X}");
    output
  })
}

fn ascii_hex_to_u8(hex: &[u8]) -> u8 {
  u8::from_str_radix(str::from_utf8(hex).expect("not valid ascii??"), 16).expect("not valid hex??")
}

fn ascii_hex_to_bin(hex: &[u8]) -> Result<Vec<u8>, std::num::ParseIntError> {
  hex
    .chunks_exact(2)
    .map(str::from_utf8)
    .filter_map(Result::ok)
    .map(|hex| u8::from_str_radix(hex, 16))
    .collect()
}
