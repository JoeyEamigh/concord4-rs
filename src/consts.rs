pub const BAUD_RATE: u32 = 9600;
pub const DATA_BITS: tokio_serial::DataBits = tokio_serial::DataBits::Eight;
pub const PARITY: tokio_serial::Parity = tokio_serial::Parity::Odd;
pub const ASCII_BYTE_REAL_LEN: usize = 2;

pub const ACK: u8 = 0x06;
pub const NAK: u8 = 0x15;

pub enum CtrlFlow {
  Ack,
  Nak,
}

impl From<u8> for CtrlFlow {
  fn from(b: u8) -> Self {
    match b {
      ACK => CtrlFlow::Ack,
      NAK => CtrlFlow::Nak,
      _ => panic!("not a u8 control character"),
    }
  }
}
