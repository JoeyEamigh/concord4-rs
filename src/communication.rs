use std::fmt::Formatter;

use crate::{
  equipment::{ArmingLevelData, FeatureState, PanelData, PartitionData, TimeDate, ZoneData, ZoneStatusData},
  touchpad::TouchpadDisplay,
};

#[derive(Debug, Clone)]
pub enum ListRequest {
  AllData = 0x00,
  ZoneData = 0x03,
  PartData = 0x04,
  BusDevData = 0x05,
  BusCapData = 0x06,
  OutputData = 0x07,
  UserData = 0x09,
  ScheduleData = 0x0a,
  EventData = 0x0b,
  LightAttach = 0x0c,
}

#[derive(Debug, Clone)]
pub enum SendableMessage {
  Ack,
  Nak,
  List(ListRequest),
  DynamicDataRefresh,
}

#[derive(Clone)]
pub enum RecvMessage {
  Ack,
  Nak,
  PanelType(PanelData),
  AutomationEventLost(Vec<u8>),
  ZoneData(ZoneData),
  PartitionData(PartitionData),
  SuperbusDevData(Vec<u8>),
  SuperbusDevCap(Vec<u8>),
  OutputData(Vec<u8>),
  EqptListDone,
  UserData(Vec<u8>),
  SchedData(Vec<u8>),
  SchedEventData(Vec<u8>),
  LightAttach(Vec<u8>),
  ClearImage(Vec<u8>),
  ZoneStatus(ZoneStatusData),
  ArmingLevel(ArmingLevelData),
  AlarmTrouble(Vec<u8>),
  EntryExitDelay(Vec<u8>),
  SirenSetup(Vec<u8>),
  SirenSync,
  SirenGo,
  Touchpad(TouchpadDisplay),
  SirenStop(Vec<u8>),
  FeatState(FeatureState),
  Temp(Vec<u8>),
  TimeAndDate(TimeDate),
  LightsState(Vec<u8>),
  UserLights(Vec<u8>),
  Keyfob(Vec<u8>),
}

impl std::fmt::Debug for RecvMessage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      RecvMessage::Ack => write!(f, "ACK"),
      RecvMessage::Nak => write!(f, "NAK"),
      RecvMessage::PanelType(data) => write!(f, "Panel Type: {:?}", data),
      RecvMessage::AutomationEventLost(data) => write!(f, "Automation Event Lost: {:?}", data),
      RecvMessage::ZoneData(data) => write!(f, "Zone Data: {:?}", data),
      RecvMessage::PartitionData(data) => write!(f, "Partition Data: {:?}", data),
      RecvMessage::SuperbusDevData(data) => write!(f, "Superbus Device Data: {:?}", data),
      RecvMessage::SuperbusDevCap(data) => write!(f, "Superbus Device Capabilities Data: {:?}", data),
      RecvMessage::OutputData(data) => write!(f, "Output Data: {:?}", data),
      RecvMessage::EqptListDone => write!(f, "Equipment List Complete"),
      RecvMessage::UserData(data) => write!(f, "User Data: {:?}", data),
      RecvMessage::SchedData(data) => write!(f, "Schedule Data: {:?}", data),
      RecvMessage::SchedEventData(data) => write!(f, "Scheduled Event Data: {:?}", data),
      RecvMessage::LightAttach(data) => write!(f, "Light to Sensor Attachment: {:?}", data),
      RecvMessage::ClearImage(data) => write!(f, "Clear Automation Image: {:?}", data),
      RecvMessage::ZoneStatus(data) => write!(f, "Zone Status: {:?}", data),
      RecvMessage::ArmingLevel(data) => write!(f, "Arming Level: {:?}", data),
      RecvMessage::AlarmTrouble(data) => write!(f, "Alarm/Trouble: {:?}", data),
      RecvMessage::EntryExitDelay(data) => write!(f, "Entry/Exit Delay: {:?}", data),
      RecvMessage::SirenSetup(data) => write!(f, "Siren Setup: {:?}", data),
      RecvMessage::SirenSync => write!(f, "Siren Synchronize"),
      RecvMessage::SirenGo => write!(f, "Siren Go"),
      RecvMessage::Touchpad(data) => write!(f, "Touchpad Display: {:?}", data),
      RecvMessage::SirenStop(data) => write!(f, "Siren Stop: {:?}", data),
      RecvMessage::FeatState(data) => write!(f, "Feature State: {:?}", data),
      RecvMessage::Temp(data) => write!(f, "Temperature: {:?}", data),
      RecvMessage::TimeAndDate(data) => write!(f, "Time and Date: {:?}", data),
      RecvMessage::LightsState(data) => write!(f, "Lights State Command: {:?}", data),
      RecvMessage::UserLights(data) => write!(f, "User Lights Command: {:?}", data),
      RecvMessage::Keyfob(data) => write!(f, "Keyfob Command: {:?}", data),
    }
  }
}

impl TryFrom<Vec<u8>> for RecvMessage {
  type Error = ();

  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    let cmd = value[0];
    let subcmd = if value.len() > 1 { Some(value[1]) } else { None };

    if let Some(subcmd) = subcmd {
      let data = value[2..].to_vec();

      let message = match (cmd, subcmd) {
        (0x22, 0x01) => Ok(RecvMessage::ArmingLevel(ArmingLevelData::from(data))),
        (0x22, 0x02) => Ok(RecvMessage::AlarmTrouble(data)),
        (0x22, 0x03) => Ok(RecvMessage::EntryExitDelay(data)),
        (0x22, 0x04) => Ok(RecvMessage::SirenSetup(data)),
        (0x22, 0x05) => Ok(RecvMessage::SirenSync),
        (0x22, 0x06) => Ok(RecvMessage::SirenGo),
        (0x22, 0x09) => Ok(RecvMessage::Touchpad(TouchpadDisplay::from(data))),
        (0x22, 0x0b) => Ok(RecvMessage::SirenStop(data)),
        (0x22, 0x0c) => Ok(RecvMessage::FeatState(FeatureState::from(data))),
        (0x22, 0x0d) => Ok(RecvMessage::Temp(data)),
        (0x22, 0x0e) => Ok(RecvMessage::TimeAndDate(TimeDate::from(data))),
        (0x23, 0x01) => Ok(RecvMessage::LightsState(data)),
        (0x23, 0x02) => Ok(RecvMessage::UserLights(data)),
        (0x23, 0x03) => Ok(RecvMessage::Keyfob(data)),
        _ => Err(()),
      };

      if message.is_ok() {
        return message;
      }
    }

    let data = value[1..].to_vec();

    match cmd {
      0x01 => Ok(RecvMessage::PanelType(PanelData::from(data))),
      0x02 => Ok(RecvMessage::AutomationEventLost(data)),
      0x03 => Ok(RecvMessage::ZoneData(ZoneData::from(data))),
      0x04 => Ok(RecvMessage::PartitionData(PartitionData::from(data))),
      0x05 => Ok(RecvMessage::SuperbusDevData(data)),
      0x06 => Ok(RecvMessage::SuperbusDevCap(data)),
      0x07 => Ok(RecvMessage::OutputData(data)),
      0x08 => Ok(RecvMessage::EqptListDone),
      0x09 => Ok(RecvMessage::UserData(data)),
      0x0a => Ok(RecvMessage::SchedData(data)),
      0x0b => Ok(RecvMessage::SchedEventData(data)),
      0x0c => Ok(RecvMessage::LightAttach(data)),
      0x20 => Ok(RecvMessage::ClearImage(data)),
      0x21 => Ok(RecvMessage::ZoneStatus(ZoneStatusData::from(data))),
      0x23 => Ok(RecvMessage::LightsState(data)),
      _ => Err(()),
    }
  }
}
