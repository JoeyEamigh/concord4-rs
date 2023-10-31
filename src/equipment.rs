use crate::decode;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "lowercase"))]
pub enum ZoneStatus {
  Normal,
  Tripped,
  Faulted,
  Alarm,
  Trouble,
  Bypassed,
  Unknown,
}

impl From<u8> for ZoneStatus {
  fn from(data: u8) -> Self {
    match data {
      0x0 => ZoneStatus::Normal,
      0x1 => ZoneStatus::Tripped,
      0x2 => ZoneStatus::Faulted,
      0x4 => ZoneStatus::Alarm,
      0x8 => ZoneStatus::Trouble,
      0xA => ZoneStatus::Bypassed,
      _ => ZoneStatus::Unknown,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "lowercase"))]
pub enum ZoneType {
  Hardwired,
  RF,
  Touchpad,
}

impl From<u8> for ZoneType {
  fn from(data: u8) -> Self {
    match data {
      0x0 => ZoneType::Hardwired,
      0x1 => ZoneType::RF,
      0x2 => ZoneType::Touchpad,
      _ => ZoneType::Hardwired,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct ZoneData {
  pub id: String,
  pub partition_number: u8,
  pub area_number: u8,
  pub group_number: u8,
  pub zone_number: u8,
  pub zone_type: ZoneType,
  pub zone_status: ZoneStatus,
  pub zone_text: String,
}

impl From<Vec<u8>> for ZoneData {
  fn from(data: Vec<u8>) -> Self {
    ZoneData {
      id: format!("p{}-z{}", data[0], data[4]),
      partition_number: data[0],
      area_number: data[1],
      group_number: data[2],
      zone_number: data[4],
      zone_type: ZoneType::from(data[5]),
      zone_status: ZoneStatus::from(data[6]),
      zone_text: decode::decode_text_tokens(&data[7..]),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ZoneStatusData {
  pub id: String,
  pub partition_number: u8,
  pub area_number: u8,
  pub zone_number: u8,
  pub zone_status: ZoneStatus,
}

impl From<Vec<u8>> for ZoneStatusData {
  fn from(data: Vec<u8>) -> Self {
    ZoneStatusData {
      id: format!("p{}-z{}", data[0], data[3]),
      partition_number: data[0],
      area_number: data[1],
      zone_number: data[3],
      zone_status: ZoneStatus::from(data[4]),
    }
  }
}

#[derive(Debug, Clone)]
pub enum PartitionArmingLevel {
  Off,
  Stay,
  Away,
  PhoneTest,
  SensorTest,
}

impl From<u8> for PartitionArmingLevel {
  fn from(value: u8) -> Self {
    match value {
      0x1 => PartitionArmingLevel::Off,
      0x2 => PartitionArmingLevel::Stay,
      0x3 => PartitionArmingLevel::Away,
      0x8 => PartitionArmingLevel::PhoneTest,
      0x9 => PartitionArmingLevel::SensorTest,
      _ => PartitionArmingLevel::Off,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct PartitionData {
  pub id: String,
  pub partition_number: u8,
  pub area_number: u8,
  pub arming_level: ArmingLevel,
}

impl From<Vec<u8>> for PartitionData {
  fn from(data: Vec<u8>) -> Self {
    PartitionData {
      id: format!("p{}", data[0]),
      partition_number: data[0],
      area_number: data[1],
      arming_level: ArmingLevel::from(PartitionArmingLevel::from(data[2])),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "lowercase"))]
pub enum PanelType {
  Concord,
  ConcordExpress,
  ConcordExpress4,
  ConcordEuro,
}

impl From<u8> for PanelType {
  fn from(value: u8) -> Self {
    match value {
      0x14 => PanelType::Concord,
      0x0b => PanelType::ConcordExpress,
      0x1e => PanelType::ConcordExpress4,
      0x0e => PanelType::ConcordEuro,
      _ => PanelType::Concord,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct PanelData {
  pub panel_type: PanelType,
}

impl Default for PanelData {
  fn default() -> Self {
    PanelData {
      panel_type: PanelType::Concord,
    }
  }
}

impl From<Vec<u8>> for PanelData {
  fn from(data: Vec<u8>) -> Self {
    PanelData {
      panel_type: PanelType::from(data[0]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub enum ArmingLevel {
  ZoneTest,
  Off,
  Home,
  Away,
  Night,
  Silent,
}

impl From<u8> for ArmingLevel {
  fn from(value: u8) -> Self {
    match value {
      0x0 => ArmingLevel::ZoneTest,
      0x1 => ArmingLevel::Off,
      0x2 => ArmingLevel::Home,
      0x3 => ArmingLevel::Away,
      0x4 => ArmingLevel::Night,
      0x5 => ArmingLevel::Silent,
      _ => ArmingLevel::Off,
    }
  }
}

impl From<PartitionArmingLevel> for ArmingLevel {
  fn from(value: PartitionArmingLevel) -> Self {
    match value {
      PartitionArmingLevel::Stay => ArmingLevel::Home,
      PartitionArmingLevel::Away => ArmingLevel::Away,
      PartitionArmingLevel::PhoneTest => ArmingLevel::ZoneTest,
      PartitionArmingLevel::SensorTest => ArmingLevel::ZoneTest,
      PartitionArmingLevel::Off => ArmingLevel::Off,
    }
  }
}

#[derive(Debug, Clone)]
pub struct ArmingLevelData {
  pub id: String,
  pub partition_number: u8,
  pub area_number: u8,
  pub arming_level: ArmingLevel,
}

impl From<Vec<u8>> for ArmingLevelData {
  fn from(data: Vec<u8>) -> Self {
    ArmingLevelData {
      id: format!("p{}", data[0]),
      partition_number: data[0],
      area_number: data[1],
      arming_level: ArmingLevel::from(data[4]),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Feature {
  Chime,
  EnergySaver,
  NoDelay,
  LatchKey,
  SilentArm,
  QuickArm,
}

impl From<u8> for Feature {
  fn from(value: u8) -> Self {
    match value {
      0x01 => Feature::Chime,
      0x02 => Feature::EnergySaver,
      0x04 => Feature::NoDelay,
      0x08 => Feature::LatchKey,
      0x10 => Feature::SilentArm,
      0x20 => Feature::QuickArm,
      _ => Feature::Chime,
    }
  }
}

#[derive(Debug, Clone)]
pub struct FeatureState {
  pub partition_number: u8,
  pub area_number: u8,
  pub feature_state: Feature,
}

impl From<Vec<u8>> for FeatureState {
  fn from(data: Vec<u8>) -> Self {
    FeatureState {
      partition_number: data[0],
      area_number: data[1],
      feature_state: Feature::from(data[2]),
    }
  }
}

#[derive(Debug, Clone)]
pub struct TimeDate {
  pub hour: u8,
  pub minute: u8,
  pub month: u8,
  pub day: u8,
  pub year: u8,
}

impl From<Vec<u8>> for TimeDate {
  fn from(data: Vec<u8>) -> Self {
    TimeDate {
      hour: data[0],
      minute: data[1],
      month: data[2],
      day: data[3],
      year: data[4],
    }
  }
}
