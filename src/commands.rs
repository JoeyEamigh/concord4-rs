#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub enum Keypress {
  #[cfg_attr(feature = "json", serde(rename = "0"))]
  Zero,
  #[cfg_attr(feature = "json", serde(rename = "1"))]
  One,
  #[cfg_attr(feature = "json", serde(rename = "2"))]
  Two,
  #[cfg_attr(feature = "json", serde(rename = "3"))]
  Three,
  #[cfg_attr(feature = "json", serde(rename = "4"))]
  Four,
  #[cfg_attr(feature = "json", serde(rename = "5"))]
  Five,
  #[cfg_attr(feature = "json", serde(rename = "6"))]
  Six,
  #[cfg_attr(feature = "json", serde(rename = "7"))]
  Seven,
  #[cfg_attr(feature = "json", serde(rename = "8"))]
  Eight,
  #[cfg_attr(feature = "json", serde(rename = "9"))]
  Nine,
  #[cfg_attr(feature = "json", serde(rename = "*"))]
  Star,
  #[cfg_attr(feature = "json", serde(rename = "#"))]
  Pound,
  PolicePanic,
  AuxPanic,
  FirePanic,
  LightsOn,
  LightsOff,
  LightsToggle,
  KeyswitchOn,
  KeyswitchOff,
  KeyswitchToggle, // docs say not implemented??
  // 0x16 - 0x1b are undefined
  FireTPAcknowledge,
  FireTPSilence,
  FireTPFireTest,
  FireTPSmokeReset,
  KeyfobDisarm,
  KeyfobArm,
  KeyfobLights,
  KeyfobStar,
  KeyfobArmDisarm,
  KeyfobLightsStar,
  KeyfobLongLights,
  KeyfobDirectArmLevelThree,
  KeyfobDirectArmLevelTwo,
  KeyfobArmStar,
  KeyfobDisarmLights,
  TPAKey,
  TPBKey,
  TPCKey,
  TPDKey,
  TPEKey,
  TPFKey,
}

impl From<Keypress> for u8 {
  fn from(value: Keypress) -> Self {
    match value {
      Keypress::Zero => 0x00,
      Keypress::One => 0x01,
      Keypress::Two => 0x02,
      Keypress::Three => 0x03,
      Keypress::Four => 0x04,
      Keypress::Five => 0x05,
      Keypress::Six => 0x06,
      Keypress::Seven => 0x07,
      Keypress::Eight => 0x08,
      Keypress::Nine => 0x09,
      Keypress::Star => 0x0a,
      Keypress::Pound => 0x0b,
      Keypress::PolicePanic => 0x0c,
      Keypress::AuxPanic => 0x0d,
      Keypress::FirePanic => 0x0e,
      Keypress::LightsOn => 0x10,
      Keypress::LightsOff => 0x11,
      Keypress::LightsToggle => 0x12,
      Keypress::KeyswitchOn => 0x13,
      Keypress::KeyswitchOff => 0x14,
      Keypress::KeyswitchToggle => 0x15,
      Keypress::FireTPAcknowledge => 0x1c,
      Keypress::FireTPSilence => 0x1d,
      Keypress::FireTPFireTest => 0x1e,
      Keypress::FireTPSmokeReset => 0x1f,
      Keypress::KeyfobDisarm => 0x20,
      Keypress::KeyfobArm => 0x21,
      Keypress::KeyfobLights => 0x22,
      Keypress::KeyfobStar => 0x23,
      Keypress::KeyfobArmDisarm => 0x24,
      Keypress::KeyfobLightsStar => 0x25,
      Keypress::KeyfobLongLights => 0x26,
      Keypress::KeyfobDirectArmLevelThree => 0x27,
      Keypress::KeyfobDirectArmLevelTwo => 0x28,
      Keypress::KeyfobArmStar => 0x29,
      Keypress::KeyfobDisarmLights => 0x2a,
      Keypress::TPAKey => 0x2c,
      Keypress::TPBKey => 0x30,
      Keypress::TPCKey => 0x2d,
      Keypress::TPDKey => 0x33,
      Keypress::TPEKey => 0x2e,
      Keypress::TPFKey => 0x36,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
pub enum ListRequest {
  AllData,
  ZoneData,
  PartData,
  BusDevData,
  BusCapData,
  OutputData,
  UserData,
  ScheduleData,
  EventData,
  LightAttach,
}

impl From<u8> for ListRequest {
  fn from(value: u8) -> Self {
    match value {
      0x00 => ListRequest::AllData,
      0x03 => ListRequest::ZoneData,
      0x04 => ListRequest::PartData,
      0x05 => ListRequest::BusDevData,
      0x06 => ListRequest::BusCapData,
      0x07 => ListRequest::OutputData,
      0x09 => ListRequest::UserData,
      0x0a => ListRequest::ScheduleData,
      0x0b => ListRequest::EventData,
      0x0c => ListRequest::LightAttach,
      _ => ListRequest::AllData,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
/// The different modes the alarm can be armed to
pub enum ArmMode {
  /// Stay mode
  Stay,
  /// Away mode
  Away,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
/// The different levels the alarm can be armed to
pub enum ArmLevel {
  /// the standard level (per panel)
  Normal,
  /// silent arming
  Silent,
  /// instant arming
  Instant,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
/// Options for arming the alarm
pub struct ArmOptions {
  /// the mode to arm to
  pub mode: ArmMode,
  /// the code to disarm with
  pub code: [Keypress; 4],
  /// the level to arm to (default: [ArmLevel::Normal])
  pub level: Option<ArmLevel>,
  /// the partition to arm (default: 1)
  pub partition: Option<u8>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
/// Options for disarming the alarm
pub struct DisarmOptions {
  /// the code to disarm with
  pub code: [Keypress; 4],
  /// the partition to disarm
  pub partition: Option<u8>,
}
