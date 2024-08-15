#![doc = include_str!("../README.md")]

use serial::Serial;
use tokio::sync::mpsc;

mod commands;
mod communication;
mod consts;
mod decode;
mod equipment;
mod serial;
mod state;
mod touchpad;

pub use commands::{ArmLevel, ArmMode, ArmOptions, DisarmOptions, Keypress, ListRequest};
pub use communication::{RecvMessage, SendableMessage};
pub use equipment::{ArmingLevel, PanelData, PartitionData, ZoneData};
pub use state::{ConcordState as ConcordStateInner, WrappedState as ConcordState};

/// Struct representing a connection to a Concord4 panel.
/// Contains the current state of the alarm panel and methods to interact with it.
///
/// call `Concord4::open` to create a new connection.
pub struct Concord4 {
  /// The current state of the server. The state is Send + Sync, so it can be shared between threads.
  pub state: ConcordState,

  // internal
  serial: Serial,
}

impl Concord4 {
  /// open a new connection to a Concord4 server
  ///
  /// # args
  /// `path`: [&str] - the path to the serial port
  ///
  /// # returns
  /// a new [Concord4] struct
  ///
  /// # example
  /// ```no_run
  /// let mut client = Concord4::open("/dev/ttyUSB0").await;
  /// ```
  pub async fn open(path: &str) -> Result<Self, ClientError> {
    let state = ConcordState::default();
    let serial = Serial::init(path).await?;

    Ok(Self { state, serial })
  }

  /// send a raw command to the Concord4 panel
  ///
  /// # args
  /// `command`: [SendableMessage] - the command to send
  ///
  /// # returns
  /// an empty [Ok] if the command was sent successfully, or a [ClientError] if there was an error
  ///
  /// # example
  /// ```no_run
  /// client.send(SendableMessage::List(ListRequest::AllData)).await.expect("could not send command");
  /// ```
  pub async fn send(&mut self, message: SendableMessage) -> Result<(), ClientError> {
    self.serial.tx.send(message).await.map_err(ClientError::Sender)
  }

  /// receive a message from the Concord4 panel
  ///
  /// uses a [futures::stream::Next] under the hood, so: \
  /// creates a future that resolves to the next item in the stream
  ///
  /// # returns
  /// an [Option] containing an [Ok] with a [RecvMessage] if a message was received, \
  /// an [Option] containing an [Err] with a [ClientError] if there was an error, \
  /// or [None] if the serial port was closed
  ///
  /// # example
  /// ```no_run
  /// let message = client.recv().await.expect("could not receive message");
  /// ```
  pub async fn recv(&mut self) -> Option<Result<RecvMessage, ClientError>> {
    use futures::StreamExt;

    let message = self.serial.next().await?;

    if let Ok(message) = &message {
      self.state.handle_result(message.clone());
    }

    Some(message)
  }

  /// arm the alarm
  ///
  /// # args
  /// `options`: [ArmOptions] - the options for arming the alarm
  ///
  /// # returns
  /// an empty [Ok] if the command was sent successfully, or a [ClientError] if there was an error
  ///
  /// # example
  /// ```no_run
  /// client.arm(ArmOptions {
  ///   mode: ArmMode::Stay,
  ///   code: [Keypress::One, Keypress::Two, Keypress::Three, Keypress::Four],
  ///   level: Some(ArmLevel::Instant),
  ///   partition: Some(1),
  /// }).await.expect("could not arm alarm");
  /// ```
  pub async fn arm(&mut self, options: ArmOptions) -> Result<(), ClientError> {
    let partition = options.partition.unwrap_or(1);
    if let Some(partition_data) = self.state.partitions.get(&partition) {
      if partition_data.arming_level != ArmingLevel::Off {
        return Err(ClientError::Armed);
      }
    }

    self.send(SendableMessage::Arm(options)).await
  }

  /// disarm the alarm
  ///
  /// # args
  /// `options`: [DisarmOptions] - the options for disarming the alarm
  ///
  /// # returns
  /// an empty [Ok] if the command was sent successfully, or a [ClientError] if there was an error
  ///
  /// # example
  /// ```no_run
  /// client.disarm(DisarmOptions {
  ///   code: [Keypress::One, Keypress::Two, Keypress::Three, Keypress::Four],
  ///   partition: Some(1),
  /// }).await.expect("could not disarm alarm");
  /// ```
  pub async fn disarm(&mut self, options: DisarmOptions) -> Result<(), ClientError> {
    self.send(SendableMessage::Disarm(options)).await
  }

  /// toggle the chime on the alarm
  ///
  /// # args
  /// `partition`: [Option<u8>] - the partition to toggle the chime on, or None for partition 1
  ///
  /// # returns
  /// an empty [Ok] if the command was sent successfully, or a [ClientError] if there was an error
  ///
  /// # example
  /// ```no_run
  /// client.toggle_chime(Some(1)).await.expect("could not toggle chime");
  /// ```
  pub async fn toggle_chime(&mut self, partition: Option<u8>) -> Result<(), ClientError> {
    let partition = partition.unwrap_or(1);

    if let Some(partition_data) = self.state.partitions.get(&partition) {
      if partition_data.arming_level != ArmingLevel::Off {
        return Err(ClientError::Armed);
      }
    }

    self.send(SendableMessage::ToggleChime(Some(partition))).await
  }
}

/// Error type for the Concord4 client
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
  /// An error denoting the alarm is armed but must be disarmed for this action
  #[error("Alarm is armed; disarm first")]
  Armed,
  /// An error returned by the Encoder
  #[error("Encoder error: {0}")]
  Encoder(std::io::Error),
  /// An error returned by the Decoder
  #[error("Decoder error: {0}")]
  Decoder(std::io::Error),
  /// An error returned by the Sender
  #[error("Sender error: {0}")]
  Sender(#[from] mpsc::error::SendError<SendableMessage>),
  /// An error returned by the Receiver
  #[error("Receiver error: {0}")]
  Receiver(std::io::Error),
  /// An error returned by the Serial port
  #[error("Serial port error: {0}")]
  SerialPort(#[from] tokio_serial::Error),
  /// A serial port error
  #[error("Serial port closed")]
  SerialPortClosed,
  /// An unknown error
  #[error("Unknown error: {0}")]
  Unknown(String),
}
