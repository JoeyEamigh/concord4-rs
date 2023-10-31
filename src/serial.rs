use crate::{
  communication::{ListRequest, RecvMessage, SendableMessage},
  consts, Concord4,
};
use std::{io, str};
use tokio_util::{
  bytes::{Buf, BytesMut},
  codec::{Decoder, Encoder},
};
use tracing::{debug, error, trace, warn};

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

fn ascii_hex_to_utf8(hex: &[u8]) -> &str {
  str::from_utf8(hex).expect("not valid ascii??")
}

fn ascii_hex_to_u8(hex: &[u8]) -> u8 {
  u8::from_str_radix(ascii_hex_to_utf8(hex), 16).expect("not valid hex??")
}

fn ascii_hex_to_bin(hex: &[u8]) -> Vec<u8> {
  let mut bin = vec![];

  for i in (0..hex.len()).step_by(2) {
    bin.push(ascii_hex_to_u8(&hex[i..(i + 2)]));
  }

  bin
}

fn handle_buf_len(buf: &mut BytesMut, len: usize) {
  use tokio_util::bytes::BufMut;

  if buf.remaining_mut() < len {
    buf.reserve(len);
  }
}

const BYTE: usize = consts::ASCII_BYTE_REAL_LEN;

impl Decoder for Concord4 {
  type Item = RecvMessage;
  type Error = io::Error;

  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    if src.is_empty() {
      return Ok(None);
    }

    trace!("src: {:?}", src);

    let ctrl_ctr = src.as_ref().iter().position(|b| *b == consts::ACK || *b == consts::NAK);
    if let Some(ctrl_pos) = ctrl_ctr {
      let ctrl = src.get(ctrl_pos).unwrap();

      if *ctrl == consts::ACK {
        trace!("recv: ACK");
      } else {
        warn!("recv: NAK");
      }

      if self
        .ready_tx
        .as_ref()
        .expect("decoder on wrong thread")
        .try_send(consts::CtrlFlow::from(*ctrl))
        .is_err()
      {
        error!("failed to send ack");
      }

      let mut after_ctrl = src.split_off(ctrl_pos);
      after_ctrl.advance(1);
      src.extend(after_ctrl);
    }

    let newline = src.as_ref().iter().position(|b| *b == b'\n');

    if let Some(lf_pos) = newline {
      // post_lf stands for position after line feed
      let post_lf = lf_pos + 1;

      // byte here stands for the byte denoting length of the message
      if src.len() < post_lf + BYTE {
        trace!("message not finished - waiting for more data");
        return Ok(None);
      }

      // byte here stands for the byte denoting length of the message since length doesn't include itself
      let (data_len, data_len_in_buffer) = if let Some(len_bytes) = src.get((post_lf)..(post_lf + BYTE)) {
        let data_len = ascii_hex_to_u8(len_bytes);
        // byte here stands for how each message in buffer is twice as long as the actual data
        let data_len_in_buffer = data_len as usize * BYTE;

        trace!("data_len: {} hex bytes; {} real bytes;", data_len, data_len_in_buffer);

        (data_len, data_len_in_buffer)
      } else {
        (0, 0)
      };

      // byte here stands for the byte denoting length of the message
      if data_len_in_buffer == 0 || src.len() < post_lf + BYTE + data_len_in_buffer {
        trace!("message not finished - waiting for more data");
        return Ok(None);
      }

      src.advance(post_lf);
      // byte here stands for the byte denoting length of the message since length doesn't include itself
      let full_data = src.split_to(BYTE + data_len_in_buffer);
      debug!("message received - {:?}", full_data);

      // byte here stands for the byte denoting length of the message
      let mut data = ascii_hex_to_bin(full_data.get(BYTE..).unwrap());
      let checksum = data.pop().unwrap();

      trace!("data: {:?}", data);
      trace!("checksum: {:?}", checksum);

      if validate_checksum(data_len, &data, checksum) {
        trace!("checksum valid");

        if self.tx.try_send(SendableMessage::Ack).is_err() {
          error!("failed to send ack");
        }
      } else {
        error!("invalid checksum");

        if self.tx.try_send(SendableMessage::Nak).is_err() {
          error!("failed to send nck");
        }

        return Ok(None);
      }

      let data = RecvMessage::try_from(data);
      if let Ok(data) = data {
        return Ok(Some(data));
      } else {
        error!("failed to parse message");
      }
    }

    Ok(None)
  }
}

impl Encoder<SendableMessage> for Concord4 {
  type Error = io::Error;

  fn encode(&mut self, item: SendableMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
    use tokio_util::bytes::BufMut;
    trace!("sending: {:?}", item);

    // first byte is length so zero for now
    let mut data: Vec<u8> = vec![0x0];

    match item {
      SendableMessage::Ack => {
        handle_buf_len(dst, 1);
        dst.put_u8(consts::ACK);

        return Ok(());
      }
      SendableMessage::Nak => {
        handle_buf_len(dst, 1);
        dst.put_u8(consts::NAK);

        return Ok(());
      }
      SendableMessage::List(req) => match req {
        ListRequest::AllData => {
          data.push(0x2);
        }
        other => {
          data.push(0x2);
          data.push(other as u8);
        }
      },
      SendableMessage::DynamicDataRefresh => {
        data.push(0x20);
      }
    }

    // the length of the message doesn't include the length byte itself
    data[0] = data.len() as u8;
    data.push(compute_checksum(&data));
    let msg = ascii_hex_to_string(data.as_ref());
    let msg_with_lf = ["\n", &msg].concat();

    trace!("sending message: {:?}", msg_with_lf);

    // need to add one for the line feed
    handle_buf_len(dst, msg_with_lf.len());

    dst.put(msg_with_lf.as_bytes());

    Ok(())
  }
}
