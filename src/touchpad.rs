use crate::decode;

#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct TouchpadDisplay {
  pub partition_number: u8,
  pub area_number: u8,
  pub message_type: u8, // 0 = normal, 1 = broadcast
  pub display_tokens: Vec<u8>,
  pub text: String,
}

impl From<Vec<u8>> for TouchpadDisplay {
  fn from(data: Vec<u8>) -> Self {
    TouchpadDisplay {
      partition_number: data[0],
      area_number: data[1],
      message_type: data[2],
      display_tokens: data[3..].to_vec(),
      text: decode::decode_text_tokens(&data[3..]),
    }
  }
}
