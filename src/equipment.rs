use std::collections::HashSet;

use crate::decode;

#[cfg(feature = "json")]
use serde::Serialize;

pub trait StringIdentifiable {
  fn id(&self) -> String;
}

pub trait IntIdentifiable {
  fn id(&self) -> u8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "lowercase"))]
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
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct ZoneData {
  pub partition_number: u8,
  pub area_number: u8,
  pub group_number: u8,
  pub zone_number: u8,
  pub zone_type: ZoneType,
  pub zone_status: ZoneStatus,
  pub zone_text: String,
}

impl ZoneData {
  pub fn group_id(&self) -> String {
    format!("p{}-g{}", self.partition_number, self.group_number)
  }
}

impl StringIdentifiable for ZoneData {
  fn id(&self) -> String {
    format!("p{}-z{}", self.partition_number, self.zone_number)
  }
}

impl From<Vec<u8>> for ZoneData {
  fn from(data: Vec<u8>) -> Self {
    ZoneData {
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
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct ZoneStatusData {
  pub partition_number: u8,
  pub area_number: u8,
  pub zone_number: u8,
  pub zone_status: ZoneStatus,
}

impl ZoneStatusData {
  pub fn zone_id(&self) -> String {
    format!("p{}-z{}", self.partition_number, self.zone_number)
  }
}

impl From<Vec<u8>> for ZoneStatusData {
  fn from(data: Vec<u8>) -> Self {
    ZoneStatusData {
      partition_number: data[0],
      area_number: data[1],
      zone_number: data[3],
      zone_status: ZoneStatus::from(data[4]),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
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
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct PartitionData {
  pub partition_number: u8,
  pub area_number: u8,
  pub arming_level: ArmingLevel,
  pub zones: HashSet<String>,
}

impl IntIdentifiable for PartitionData {
  fn id(&self) -> u8 {
    self.partition_number
  }
}

impl From<Vec<u8>> for PartitionData {
  fn from(data: Vec<u8>) -> Self {
    PartitionData {
      partition_number: data[0],
      area_number: data[1],
      arming_level: ArmingLevel::from(PartitionArmingLevel::from(data[2])),
      zones: HashSet::new(),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
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
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct PanelData {
  pub panel_type: PanelType,
  pub hardware_revision: String,
  pub software_revision: String,
  pub serial_number: String,
}

impl Default for PanelData {
  fn default() -> Self {
    PanelData {
      panel_type: PanelType::Concord,
      hardware_revision: String::new(),
      software_revision: String::new(),
      serial_number: String::new(),
    }
  }
}

impl From<Vec<u8>> for PanelData {
  fn from(data: Vec<u8>) -> Self {
    PanelData {
      panel_type: PanelType::from(data[0]),
      hardware_revision: format!("{}{:X}", decode::letter_from_representative_hex(data[1]), data[2]),
      software_revision: format!("{:X}{:X}", data[3], data[4]),
      serial_number: format!("{:X}{:X}{:X}{:X}", data[5], data[6], data[7], data[8]),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
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
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct ArmingLevelData {
  pub partition_number: u8,
  pub area_number: u8,
  pub arming_level: ArmingLevel,
}

impl From<Vec<u8>> for ArmingLevelData {
  fn from(data: Vec<u8>) -> Self {
    ArmingLevelData {
      partition_number: data[0],
      area_number: data[1],
      arming_level: ArmingLevel::from(data[4]),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
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
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
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
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub enum SuperBusDeviceStatus {
  Ok,
  Failed,
}

impl From<u8> for SuperBusDeviceStatus {
  fn from(data: u8) -> Self {
    match data {
      0x0 => SuperBusDeviceStatus::Ok,
      0x1 => SuperBusDeviceStatus::Failed,
      _ => SuperBusDeviceStatus::Failed,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct SuperBusDeviceData {
  pub partition_number: u8,
  pub area_number: u8,
  pub device_id: (u8, u8, u8),
  pub device_status: SuperBusDeviceStatus,
}

impl From<Vec<u8>> for SuperBusDeviceData {
  fn from(data: Vec<u8>) -> Self {
    SuperBusDeviceData {
      partition_number: data[0],
      area_number: data[1],
      device_id: (data[2], data[3], data[4]),
      device_status: SuperBusDeviceStatus::from(data[5]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "capability", content = "data")
)]
pub enum SuperBusDeviceCapabilityData {
  PowerSupervision,
  AccessControl,
  AnalogSmoke,
  AudioListenIn,
  SnapCardSupervision,
  Microburst,
  DualPhoneLine,
  EnergyManagement,
  InputZones(u8),
  PhastAutomationSystemManager,
  PhoneInterface,
  RelayOutputs(u8),
  RFReceiver,
  RFTransmitter,
  ParallelPrinter,
  Unknown, // 0x0F
  LedTouchpad,
  OneLineTwoLineBltTouchpad,
  GuiTouchpad,
  VoiceEvacuation,
  Pager,
  DownloadableCodeData,
  JTechPremisePager,
  Cryptography,
  LedDisplay,
}

impl From<&[u8]> for SuperBusDeviceCapabilityData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => SuperBusDeviceCapabilityData::PowerSupervision,
      0x01 => SuperBusDeviceCapabilityData::AccessControl,
      0x02 => SuperBusDeviceCapabilityData::AnalogSmoke,
      0x03 => SuperBusDeviceCapabilityData::AudioListenIn,
      0x04 => SuperBusDeviceCapabilityData::SnapCardSupervision,
      0x05 => SuperBusDeviceCapabilityData::Microburst,
      0x06 => SuperBusDeviceCapabilityData::DualPhoneLine,
      0x07 => SuperBusDeviceCapabilityData::EnergyManagement,
      0x08 => SuperBusDeviceCapabilityData::InputZones(data[1]),
      0x09 => SuperBusDeviceCapabilityData::PhastAutomationSystemManager,
      0x0A => SuperBusDeviceCapabilityData::PhoneInterface,
      0x0B => SuperBusDeviceCapabilityData::RelayOutputs(data[1]),
      0x0C => SuperBusDeviceCapabilityData::RFReceiver,
      0x0D => SuperBusDeviceCapabilityData::RFTransmitter,
      0x0E => SuperBusDeviceCapabilityData::ParallelPrinter,
      0x0F => SuperBusDeviceCapabilityData::Unknown,
      0x10 => SuperBusDeviceCapabilityData::LedTouchpad,
      0x11 => SuperBusDeviceCapabilityData::OneLineTwoLineBltTouchpad,
      0x12 => SuperBusDeviceCapabilityData::GuiTouchpad,
      0x13 => SuperBusDeviceCapabilityData::VoiceEvacuation,
      0x14 => SuperBusDeviceCapabilityData::Pager,
      0x15 => SuperBusDeviceCapabilityData::DownloadableCodeData,
      0x16 => SuperBusDeviceCapabilityData::JTechPremisePager,
      0x17 => SuperBusDeviceCapabilityData::Cryptography,
      0x18 => SuperBusDeviceCapabilityData::LedDisplay,
      _ => SuperBusDeviceCapabilityData::Unknown,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct SuperBusDeviceCapability {
  pub device_id: (u8, u8, u8),
  pub capability: SuperBusDeviceCapabilityData,
}

impl From<Vec<u8>> for SuperBusDeviceCapability {
  fn from(data: Vec<u8>) -> Self {
    SuperBusDeviceCapability {
      device_id: (data[0], data[1], data[2]),
      capability: SuperBusDeviceCapabilityData::from(&data[3..]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub enum CodeType {
  User(u8),
  Master(u8),
  Duress(u8),
  SystemMaster,
  Installer,
  Dealer,
  Avm,
  QuickArm,
  KeySwitch,
  System,
}

impl From<u8> for CodeType {
  fn from(data: u8) -> Self {
    match data {
      0_u8..=229_u8 => CodeType::User(data),
      230_u8..=237_u8 => CodeType::Master(data - 230),
      238_u8..=245_u8 => CodeType::Duress(data - 238),
      246 => CodeType::SystemMaster,
      247 => CodeType::Installer,
      248 => CodeType::Dealer,
      249 => CodeType::Avm,
      250 => CodeType::QuickArm,
      251 => CodeType::KeySwitch,
      252 => CodeType::System,
      _ => CodeType::User(data),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct UserData {
  pub number: (u8, u8),
  pub user_type: CodeType,
  pub code: Option<(u8, u8, u8, u8)>,
}

impl From<Vec<u8>> for UserData {
  fn from(data: Vec<u8>) -> Self {
    if data.len() > 2 {
      UserData {
        number: (data[0], data[1]),
        user_type: CodeType::from(data[1]),
        // the code is stored in BCD format
        code: Some(((data[3] >> 4), data[3] & 0x0F, (data[4] >> 4), data[4] & 0x0F)),
      }
    } else {
      UserData {
        number: (data[0], data[1]),
        user_type: CodeType::from(data[1]),
        code: None,
      }
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub enum EventSource {
  BusDevice,
  LocalPhone,
  Zone,
  System,
  RemotePhone,
}

impl From<u8> for EventSource {
  fn from(data: u8) -> Self {
    match data {
      0x0 => EventSource::BusDevice,
      0x1 => EventSource::LocalPhone,
      0x2 => EventSource::Zone,
      0x3 => EventSource::System,
      0x4 => EventSource::RemotePhone,
      _ => EventSource::BusDevice,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum AlarmEventData {
  Unspecified,
  Fire,
  FirePanic,
  Police,
  PolicePanic,
  Medical,
  MedicalPanic,
  Auxiliary,
  AuxiliaryPanic,
  Tamper,
  NoActivity,
  Suspicion,
  NotUsed,
  LowTemperature,
  HighTemperature,
  KeystrokeViolation,
  Duress,
  ExitFault,
  ExplosiveGas,
  CarbonMonoxide,
  Environmental,
  Latchkey(u8, u8),
  EquipmentTamper,
  Holdup,
  Sprinkler,
  Heat,
  SirenTamper,
  Smoke,
  RepeaterTamper,
  FirePumpActive,
  FirePumpFailure,
  FireGateValve,
  LowCO2Pressure,
  LowLiquidPressure,
  LowLiquidLevel,
  EntryExit,
  Perimeter,
  Interior,
  Near,
  WaterAlarm,
}

impl From<&[u8]> for AlarmEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => AlarmEventData::Unspecified,
      0x01 => AlarmEventData::Fire,
      0x02 => AlarmEventData::FirePanic,
      0x03 => AlarmEventData::Police,
      0x04 => AlarmEventData::PolicePanic,
      0x05 => AlarmEventData::Medical,
      0x06 => AlarmEventData::MedicalPanic,
      0x07 => AlarmEventData::Auxiliary,
      0x08 => AlarmEventData::AuxiliaryPanic,
      0x09 => AlarmEventData::Tamper,
      0x0A => AlarmEventData::NoActivity,
      0x0B => AlarmEventData::Suspicion,
      0x0C => AlarmEventData::NotUsed,
      0x0D => AlarmEventData::LowTemperature,
      0x0E => AlarmEventData::HighTemperature,
      0x0F => AlarmEventData::KeystrokeViolation,
      0x10 => AlarmEventData::Duress,
      0x11 => AlarmEventData::ExitFault,
      0x12 => AlarmEventData::ExplosiveGas,
      0x13 => AlarmEventData::CarbonMonoxide,
      0x14 => AlarmEventData::Environmental,
      0x15 => AlarmEventData::Latchkey(data[1], data[2]),
      0x16 => AlarmEventData::EquipmentTamper,
      0x17 => AlarmEventData::Holdup,
      0x18 => AlarmEventData::Sprinkler,
      0x19 => AlarmEventData::Heat,
      0x1A => AlarmEventData::SirenTamper,
      0x1B => AlarmEventData::Smoke,
      0x1C => AlarmEventData::RepeaterTamper,
      0x1D => AlarmEventData::FirePumpActive,
      0x1E => AlarmEventData::FirePumpFailure,
      0x1F => AlarmEventData::FireGateValve,
      0x20 => AlarmEventData::LowCO2Pressure,
      0x21 => AlarmEventData::LowLiquidPressure,
      0x22 => AlarmEventData::LowLiquidLevel,
      0x23 => AlarmEventData::EntryExit,
      0x24 => AlarmEventData::Perimeter,
      0x25 => AlarmEventData::Interior,
      0x26 => AlarmEventData::Near,
      0x27 => AlarmEventData::WaterAlarm,
      _ => AlarmEventData::Unspecified,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum FireEventData {
  Unspecified,
  Hardwire,
  GroundFault,
  Device,
  Supervisory,
  LowBattery,
  Tamper,
  Sam,
  PartialObscurity,
  Jam,
  ZoneAcFail,
  NU,
  NacTrouble,
  AnalogZoneTrouble,
  FireSupervisory,
  PumpFail,
  FireGateValveClosed,
  CO2PressureTrouble,
  LiquidPressureTrouble,
  LiquidLevelTrouble,
}

impl From<&[u8]> for FireEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => FireEventData::Unspecified,
      0x01 => FireEventData::Hardwire,
      0x02 => FireEventData::GroundFault,
      0x03 => FireEventData::Device,
      0x04 => FireEventData::Supervisory,
      0x05 => FireEventData::LowBattery,
      0x06 => FireEventData::Tamper,
      0x07 => FireEventData::Sam,
      0x08 => FireEventData::PartialObscurity,
      0x09 => FireEventData::Jam,
      0x0A => FireEventData::ZoneAcFail,
      0x0B => FireEventData::NU,
      0x0C => FireEventData::NacTrouble,
      0x0D => FireEventData::AnalogZoneTrouble,
      0x0E => FireEventData::FireSupervisory,
      0x0F => FireEventData::PumpFail,
      0x10 => FireEventData::FireGateValveClosed,
      0x11 => FireEventData::CO2PressureTrouble,
      0x12 => FireEventData::LiquidPressureTrouble,
      0x13 => FireEventData::LiquidLevelTrouble,
      _ => FireEventData::Unspecified,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum BypassEventData {
  DirectBypass(u8, u8),
  IndirectBypass(u8, u8),
  SwingerBypass,
  Inhibit(u8, u8),
}

impl From<&[u8]> for BypassEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => BypassEventData::DirectBypass(data[1], data[2]),
      0x01 => BypassEventData::IndirectBypass(data[1], data[2]),
      0x02 => BypassEventData::SwingerBypass,
      0x03 => BypassEventData::Inhibit(data[1], data[2]),
      _ => BypassEventData::DirectBypass(data[1], data[2]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum OpeningEventData {
  NormalOpen(u8, u8),
  EarlyOpen(u8, u8),
  LateOpen(u8, u8),
  FailToOpen,
  OpenException(u8, u8),
  OpenExtension(u8, u8),
  OpenUsingKeyfob,
  ScheduledOpen,
  RemoteOpen(u8, u8),
}

impl From<&[u8]> for OpeningEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => OpeningEventData::NormalOpen(data[1], data[2]),
      0x01 => OpeningEventData::EarlyOpen(data[1], data[2]),
      0x02 => OpeningEventData::LateOpen(data[1], data[2]),
      0x03 => OpeningEventData::FailToOpen,
      0x04 => OpeningEventData::OpenException(data[1], data[2]),
      0x05 => OpeningEventData::OpenExtension(data[1], data[2]),
      0x06 => OpeningEventData::OpenUsingKeyfob,
      0x07 => OpeningEventData::ScheduledOpen,
      0x08 => OpeningEventData::RemoteOpen(data[1], data[2]),
      _ => OpeningEventData::NormalOpen(data[1], data[2]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum ClosingEventData {
  NormalClose(u8, u8),
  EarlyClose(u8, u8),
  LateClose(u8, u8),
  FailToClose,
  CloseException(u8, u8),
  CloseExtension(u8, u8),
  CloseUsingKeyfob,
  ScheduledClose,
  RemoteClose(u8, u8),
  RecentClose(u8, u8),
}

impl From<&[u8]> for ClosingEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => ClosingEventData::NormalClose(data[1], data[2]),
      0x01 => ClosingEventData::EarlyClose(data[1], data[2]),
      0x02 => ClosingEventData::LateClose(data[1], data[2]),
      0x03 => ClosingEventData::FailToClose,
      0x04 => ClosingEventData::CloseException(data[1], data[2]),
      0x05 => ClosingEventData::CloseExtension(data[1], data[2]),
      0x06 => ClosingEventData::CloseUsingKeyfob,
      0x07 => ClosingEventData::ScheduledClose,
      0x08 => ClosingEventData::RemoteClose(data[1], data[2]),
      0x09 => ClosingEventData::RecentClose(data[1], data[2]),
      _ => ClosingEventData::NormalClose(data[1], data[2]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum PartitionConfigEventData {
  UserAccessCodeAdded(u8, u8),
  UserAccessCodeDeleted(u8, u8),
  UserAccessCodeChanged(u8, u8),
  UserAccessCodeExpired(u8, u8),
  UserCodeAuthorityChanged,
  AuthorityLevelsChanged,
  ScheduleChanged,
  ArmingOrOcScheduleChanged,
  ZoneAdded,
  ZoneDeleted,
}

impl From<&[u8]> for PartitionConfigEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => PartitionConfigEventData::UserAccessCodeAdded(data[1], data[2]),
      0x01 => PartitionConfigEventData::UserAccessCodeDeleted(data[1], data[2]),
      0x02 => PartitionConfigEventData::UserAccessCodeChanged(data[1], data[2]),
      0x03 => PartitionConfigEventData::UserAccessCodeExpired(data[1], data[2]),
      0x04 => PartitionConfigEventData::UserCodeAuthorityChanged,
      0x05 => PartitionConfigEventData::AuthorityLevelsChanged,
      0x06 => PartitionConfigEventData::ScheduleChanged,
      0x07 => PartitionConfigEventData::ArmingOrOcScheduleChanged,
      0x08 => PartitionConfigEventData::ZoneAdded,
      0x09 => PartitionConfigEventData::ZoneDeleted,
      _ => PartitionConfigEventData::UserAccessCodeAdded(data[1], data[2]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum PartitionEventData {
  ScheduleOn(u8, u8),
  ScheduleOff(u8, u8),
  LatchkeyOn,
  LatchkeyOff,
  SmokeDetectorsReset,
  ValidUserAccessCodeEntered(u8, u8),
  ArmingLevelChanged(u8, u8),
  AlarmReported,
  AgentRelease,
  AgentReleaseRestoral,
  PartitionRemoteAccess,
  KeystrokeViolationInPartition,
  ManualForceArm(u8, u8),
  AutoForceArm,
  AutoForceArmFailed,
  ArmingProtestBegun(u8, u8),
  ArmingProtestEnded(u8, u8),
}

impl From<&[u8]> for PartitionEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => PartitionEventData::ScheduleOn(data[1], data[2]),
      0x01 => PartitionEventData::ScheduleOff(data[1], data[2]),
      0x02 => PartitionEventData::LatchkeyOn,
      0x03 => PartitionEventData::LatchkeyOff,
      0x04 => PartitionEventData::SmokeDetectorsReset,
      0x05 => PartitionEventData::ValidUserAccessCodeEntered(data[1], data[2]),
      0x06 => PartitionEventData::ArmingLevelChanged(data[1], data[2]),
      0x07 => PartitionEventData::AlarmReported,
      0x08 => PartitionEventData::AgentRelease,
      0x09 => PartitionEventData::AgentReleaseRestoral,
      0x0A => PartitionEventData::PartitionRemoteAccess,
      0x0B => PartitionEventData::KeystrokeViolationInPartition,
      0x0C => PartitionEventData::ManualForceArm(data[1], data[2]),
      0x0D => PartitionEventData::AutoForceArm,
      0x0E => PartitionEventData::AutoForceArmFailed,
      0x0F => PartitionEventData::ArmingProtestBegun(data[1], data[2]),
      0x10 => PartitionEventData::ArmingProtestEnded(data[1], data[2]),
      _ => PartitionEventData::ScheduleOn(data[1], data[2]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum PartitionTestEventData {
  ManualPhoneTest(u8, u8),
  AutoPhoneTest,
  AutoPhoneTestWithExistingTrouble,
  PhoneTestOk,
  PhoneTestFailed,
  UserSensorTestStarted(u8, u8),
  UserSensorTestEnded(u8, u8),
  UserSenorTestCompleted(u8, u8),
  UserSensorTestIncomplete(u8, u8),
  UserSensorTestTrip,
  InstallerSensorTestStarted,
  InstallerSensorTestEnded,
  InstallerSensorTestCompleted,
  InstallerSensorTestIncomplete,
  InstallerSensorTestTrip,
  FireDrillStarted(u8, u8),
}

impl From<&[u8]> for PartitionTestEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => PartitionTestEventData::ManualPhoneTest(data[1], data[2]),
      0x01 => PartitionTestEventData::AutoPhoneTest,
      0x02 => PartitionTestEventData::AutoPhoneTestWithExistingTrouble,
      0x03 => PartitionTestEventData::PhoneTestOk,
      0x04 => PartitionTestEventData::PhoneTestFailed,
      0x05 => PartitionTestEventData::UserSensorTestStarted(data[1], data[2]),
      0x06 => PartitionTestEventData::UserSensorTestEnded(data[1], data[2]),
      0x07 => PartitionTestEventData::UserSenorTestCompleted(data[1], data[2]),
      0x08 => PartitionTestEventData::UserSensorTestIncomplete(data[1], data[2]),
      0x09 => PartitionTestEventData::UserSensorTestTrip,
      0x0A => PartitionTestEventData::InstallerSensorTestStarted,
      0x0B => PartitionTestEventData::InstallerSensorTestEnded,
      0x0C => PartitionTestEventData::InstallerSensorTestCompleted,
      0x0D => PartitionTestEventData::InstallerSensorTestIncomplete,
      0x0E => PartitionTestEventData::InstallerSensorTestTrip,
      0x0F => PartitionTestEventData::FireDrillStarted(data[1], data[2]),
      _ => PartitionTestEventData::ManualPhoneTest(data[1], data[2]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum SystemTroubleEventData {
  BusReceiverFailure,
  BusAntennaTamper,
  MainLowBattery,
  SnapCardLowBattery,
  ModuleLowBattery,
  MainAcFailure,
  SnapCardAcFailure,
  ModuleAcFailure,
  AuxPowerFailure,
  BusShutdown,
  BusLowPowerMode,
  PhoneLine1Failure,
  PhoneLine2Failure,
  RemotePhoneTamper,
  WatchdogReset,
  RamFailure,
  FlashFailure,
  PrinterError,
  HistoryBufferAlmostFull,
  HistoryBufferOverflow,
  ReportBufferOverflow,
  BusDeviceFailure,
  FailureToCommunicate,
  LongRangeRadioTrouble,
  ModuleTamperTrouble,
  UnenrolledModuleTrouble,
  AudioOutputTrouble,
  AnalogModuleTrouble,
  CellModuleTrouble,
  Buddy1Failure,
  Buddy2Failure,
  Buddy3Failure,
  Buddy4Failure,
  SnapCardTrouble,
  AnalogLoopShort,
  AnalogLoopBreak,
  AnalogAddress0,
  UnenrolledAnalogHead,
  DuplicateAnalogHead,
  AnalogModuleInitializing,
  MicrophoneSwitchTrouble,
  MicrophoneTrouble,
  MicrophoneWiringTrouble,
  JTechPremisePagingTrouble,
  VoiceSirenTamperTrouble,
  MicroburstTransmitFailure,
  MicroburstTransmitDisabled,
  MicroburstModuleFailure,
  MicroburstNotInService,
  AutomationSupervisoryTrouble,
  MicroburstModuleInitializing,
  PrinterPaperOutTrouble,
}

impl From<&[u8]> for SystemTroubleEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => SystemTroubleEventData::BusReceiverFailure,
      0x01 => SystemTroubleEventData::BusAntennaTamper,
      0x02 => SystemTroubleEventData::MainLowBattery,
      0x03 => SystemTroubleEventData::SnapCardLowBattery,
      0x04 => SystemTroubleEventData::ModuleLowBattery,
      0x05 => SystemTroubleEventData::MainAcFailure,
      0x06 => SystemTroubleEventData::SnapCardAcFailure,
      0x07 => SystemTroubleEventData::ModuleAcFailure,
      0x08 => SystemTroubleEventData::AuxPowerFailure,
      0x09 => SystemTroubleEventData::BusShutdown,
      0x0A => SystemTroubleEventData::BusLowPowerMode,
      0x0B => SystemTroubleEventData::PhoneLine1Failure,
      0x0C => SystemTroubleEventData::PhoneLine2Failure,
      0x0D => SystemTroubleEventData::RemotePhoneTamper,
      0x0E => SystemTroubleEventData::WatchdogReset,
      0x0F => SystemTroubleEventData::RamFailure,
      0x10 => SystemTroubleEventData::FlashFailure,
      0x11 => SystemTroubleEventData::PrinterError,
      0x12 => SystemTroubleEventData::HistoryBufferAlmostFull,
      0x13 => SystemTroubleEventData::HistoryBufferOverflow,
      0x14 => SystemTroubleEventData::ReportBufferOverflow,
      0x15 => SystemTroubleEventData::BusDeviceFailure,
      0x16 => SystemTroubleEventData::FailureToCommunicate,
      0x17 => SystemTroubleEventData::LongRangeRadioTrouble,
      0x18 => SystemTroubleEventData::ModuleTamperTrouble,
      0x19 => SystemTroubleEventData::UnenrolledModuleTrouble,
      0x1A => SystemTroubleEventData::AudioOutputTrouble,
      0x1B => SystemTroubleEventData::AnalogModuleTrouble,
      0x1C => SystemTroubleEventData::CellModuleTrouble,
      0x1D => SystemTroubleEventData::Buddy1Failure,
      0x1E => SystemTroubleEventData::Buddy2Failure,
      0x1F => SystemTroubleEventData::Buddy3Failure,
      0x20 => SystemTroubleEventData::Buddy4Failure,
      0x21 => SystemTroubleEventData::SnapCardTrouble,
      0x22 => SystemTroubleEventData::AnalogLoopShort,
      0x23 => SystemTroubleEventData::AnalogLoopBreak,
      0x24 => SystemTroubleEventData::AnalogAddress0,
      0x25 => SystemTroubleEventData::UnenrolledAnalogHead,
      0x26 => SystemTroubleEventData::DuplicateAnalogHead,
      0x27 => SystemTroubleEventData::AnalogModuleInitializing,
      0x28 => SystemTroubleEventData::MicrophoneSwitchTrouble,
      0x29 => SystemTroubleEventData::MicrophoneTrouble,
      0x2A => SystemTroubleEventData::MicrophoneWiringTrouble,
      0x2B => SystemTroubleEventData::JTechPremisePagingTrouble,
      0x2C => SystemTroubleEventData::VoiceSirenTamperTrouble,
      0x2D => SystemTroubleEventData::MicroburstTransmitFailure,
      0x2E => SystemTroubleEventData::MicroburstTransmitDisabled,
      0x2F => SystemTroubleEventData::MicroburstModuleFailure,
      0x30 => SystemTroubleEventData::MicroburstNotInService,
      0x31 => SystemTroubleEventData::AutomationSupervisoryTrouble,
      0x32 => SystemTroubleEventData::MicroburstModuleInitializing,
      0x33 => SystemTroubleEventData::PrinterPaperOutTrouble,
      _ => SystemTroubleEventData::BusReceiverFailure,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum SystemConfigChangeEventData {
  ProgramModeEntry,
  ProgramModeExitWithoutChange,
  ProgramModeExitWithChange,
  DownloaderSessionStart,
  DownloaderSessionEndWithoutChange,
  DownloaderSessionEndWithChange,
  DownloaderError,
  DownloaderConnectionDenied,
  DateTimeChanged,
  ModuleAdded,
  ModuleDeleted,
  SpeechTokensChanged,
  CodeChanged,
  PanelFirstService,
  PanelBackInService,
  InstallerCodeChanged,
}

impl From<&[u8]> for SystemConfigChangeEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => SystemConfigChangeEventData::ProgramModeEntry,
      0x01 => SystemConfigChangeEventData::ProgramModeExitWithoutChange,
      0x02 => SystemConfigChangeEventData::ProgramModeExitWithChange,
      0x03 => SystemConfigChangeEventData::DownloaderSessionStart,
      0x04 => SystemConfigChangeEventData::DownloaderSessionEndWithoutChange,
      0x05 => SystemConfigChangeEventData::DownloaderSessionEndWithChange,
      0x06 => SystemConfigChangeEventData::DownloaderError,
      0x07 => SystemConfigChangeEventData::DownloaderConnectionDenied,
      0x08 => SystemConfigChangeEventData::DateTimeChanged,
      0x09 => SystemConfigChangeEventData::ModuleAdded,
      0x0A => SystemConfigChangeEventData::ModuleDeleted,
      0x0B => SystemConfigChangeEventData::SpeechTokensChanged,
      0x0C => SystemConfigChangeEventData::CodeChanged,
      0x0D => SystemConfigChangeEventData::PanelFirstService,
      0x0E => SystemConfigChangeEventData::PanelBackInService,
      0x0F => SystemConfigChangeEventData::InstallerCodeChanged,
      _ => SystemConfigChangeEventData::ProgramModeEntry,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "subEvent", content = "userNumber")
)]
pub enum SystemEventData {
  CallbackRequested,
  OutputActivity,
  BuddyReception,
  BuddyTransmissionRequest,
  HistoryBufferCleared,
  OutputOn(u8, u8),
  OutputOff(u8, u8),
}

impl From<&[u8]> for SystemEventData {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => SystemEventData::CallbackRequested,
      0x01 => SystemEventData::OutputActivity,
      0x02 => SystemEventData::BuddyReception,
      0x03 => SystemEventData::BuddyTransmissionRequest,
      0x04 => SystemEventData::HistoryBufferCleared,
      0x05 => SystemEventData::OutputOn(data[1], data[2]),
      0x06 => SystemEventData::OutputOff(data[1], data[2]),
      _ => SystemEventData::CallbackRequested,
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(
  feature = "json",
  derive(Serialize),
  serde(rename_all = "camelCase", tag = "event", content = "data")
)]
pub enum Event {
  Alarm(AlarmEventData),
  Fire(FireEventData),
  Bypass(BypassEventData),
  Opening(OpeningEventData),
  Closing(ClosingEventData),
  PartitionConfig(PartitionConfigEventData),
  Partition(PartitionEventData),
  PartitionTest(PartitionTestEventData),
  SystemTrouble(SystemTroubleEventData),
  SystemConfigChange(SystemConfigChangeEventData),
  System(SystemEventData),
}

impl From<&[u8]> for Event {
  fn from(data: &[u8]) -> Self {
    match data[0] {
      0x00 => Event::Alarm(AlarmEventData::from(&data[1..])),
      0x01 => Event::Fire(FireEventData::from(&data[1..])),
      0x02 => Event::Bypass(BypassEventData::from(&data[1..])),
      0x03 => Event::Opening(OpeningEventData::from(&data[1..])),
      0x04 => Event::Closing(ClosingEventData::from(&data[1..])),
      0x05 => Event::PartitionConfig(PartitionConfigEventData::from(&data[1..])),
      0x06 => Event::Partition(PartitionEventData::from(&data[1..])),
      0x07 => Event::PartitionTest(PartitionTestEventData::from(&data[1..])),
      0x08 => Event::SystemTrouble(SystemTroubleEventData::from(&data[1..])),
      0x09 => Event::SystemConfigChange(SystemConfigChangeEventData::from(&data[1..])),
      0x0A => Event::System(SystemEventData::from(&data[1..])),
      _ => Event::Alarm(AlarmEventData::from(&data[1..])),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct AlarmTrouble {
  pub partition_number: u8,
  pub area_number: u8,
  pub source_type: EventSource,
  pub source_number: (u8, u8, u8),
  pub event: Event,
}

impl From<Vec<u8>> for AlarmTrouble {
  fn from(data: Vec<u8>) -> Self {
    AlarmTrouble {
      partition_number: data[0],
      area_number: data[1],
      source_type: EventSource::from(data[2]),
      source_number: (data[3], data[4], data[5]),
      event: Event::from(&data[6..]),
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct SirenStop {
  partition_number: u8,
  area_number: u8,
}

impl From<Vec<u8>> for SirenStop {
  fn from(data: Vec<u8>) -> Self {
    SirenStop {
      partition_number: data[0],
      area_number: data[1],
    }
  }
}
