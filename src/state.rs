use dashmap::DashMap;
use std::{
  collections::HashSet,
  sync::{Arc, OnceLock},
};

#[cfg(feature = "json")]
use serde::Serialize;

use crate::{
  communication::RecvMessage,
  equipment::{
    ArmingLevelData, IntIdentifiable, PanelData, PartitionData, StringIdentifiable, ZoneData, ZoneStatusData,
  },
};

pub type WrappedState = Arc<ConcordState>;

#[derive(Debug, Clone, Default)]
pub struct WrappedPanel(pub OnceLock<PanelData>);

#[cfg(feature = "json")]
impl Serialize for WrappedPanel {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    if let Some(panel) = self.0.get() {
      panel.serialize(serializer)
    } else {
      PanelData::default().serialize(serializer)
    }
  }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct Group {
  pub partition_number: u8,
  pub number: u8,
  pub zones: HashSet<String>,
}

impl Group {
  pub fn new(partition_number: u8, number: u8) -> Self {
    Self {
      partition_number,
      number,
      zones: HashSet::new(),
    }
  }
}

impl StringIdentifiable for Group {
  fn id(&self) -> String {
    format!("p{}-g{}", self.partition_number, self.number)
  }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct ConcordState {
  pub panel: WrappedPanel,
  pub zones: DashMap<String, ZoneData>,
  pub partitions: DashMap<u8, PartitionData>,
  pub groups: DashMap<String, Group>,
}

impl ConcordState {
  #[cfg(feature = "json")]
  pub fn to_json(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string(&self)
  }

  pub(crate) fn handle_result(&self, data: RecvMessage) {
    match data {
      RecvMessage::Ack => {}
      RecvMessage::Nak => {}
      RecvMessage::PanelType(data) => self.handle_panel_type(data),
      RecvMessage::ZoneData(data) => self.handle_zone_data(data),
      RecvMessage::ZoneStatus(data) => self.handle_zone_status(data),
      RecvMessage::PartitionData(data) => self.handle_partition_data(data),
      RecvMessage::ArmingLevel(data) => self.handle_arming_level(data),
      RecvMessage::EqptListDone => {
        tracing::trace!(target: "concord4::state::eqpt-list-done", "unhandled: {:?}", data);
      }
      RecvMessage::SirenSync => {
        tracing::trace!(target: "concord4::state::siren-sync", "unhandled: {:?}", data);
      }
      RecvMessage::Touchpad(_) => {
        tracing::trace!(target: "concord4::state::touchpad", "unhandled: {:?}", data);
      }
      RecvMessage::AutomationEventLost(_) => {
        tracing::trace!(target: "concord4::state::automation-event-lost", "unhandled: {:?}", data);
      }
      RecvMessage::SuperBusDevData(_) => {
        tracing::trace!(target: "concord4::state::superbus-dev-data", "unhandled: {:?}", data);
      }
      RecvMessage::SuperBusDevCap(_) => {
        tracing::trace!(target: "concord4::state::superbus-dev-cap", "unhandled: {:?}", data);
      }
      RecvMessage::OutputData(_) => {
        tracing::trace!(target: "concord4::state::output-data", "unhandled: {:?}", data);
      }
      RecvMessage::UserData(_) => {
        tracing::trace!(target: "concord4::state::user-data", "unhandled: {:?}", data);
      }
      RecvMessage::SchedData(_) => {
        tracing::trace!(target: "concord4::state::sched-data", "unhandled: {:?}", data);
      }
      RecvMessage::SchedEventData(_) => {
        tracing::trace!(target: "concord4::state::sched-event-data", "unhandled: {:?}", data);
      }
      RecvMessage::LightAttach(_) => {
        tracing::trace!(target: "concord4::state::light-attach", "unhandled: {:?}", data);
      }
      RecvMessage::ClearImage(_) => {
        tracing::trace!(target: "concord4::state::clear-image", "unhandled: {:?}", data);
      }
      RecvMessage::AlarmTrouble(_) => {
        tracing::trace!(target: "concord4::state::alarm-trouble", "unhandled: {:?}", data);
      }
      RecvMessage::EntryExitDelay(_) => {
        tracing::trace!(target: "concord4::state::entry-exit-delay", "unhandled: {:?}", data);
      }
      RecvMessage::SirenSetup(_) => {
        tracing::trace!(target: "concord4::state::siren-setup", "unhandled: {:?}", data);
      }
      RecvMessage::SirenGo => {
        tracing::trace!(target: "concord4::state::siren-go", "unhandled: {:?}", data);
      }
      RecvMessage::SirenStop(_) => {
        tracing::trace!(target: "concord4::state::siren-stop", "unhandled: {:?}", data);
      }
      RecvMessage::FeatState(_) => {
        tracing::trace!(target: "concord4::state::feat-state", "unhandled: {:?}", data);
      }
      RecvMessage::Temp(_) => {
        tracing::trace!(target: "concord4::state::temp", "unhandled: {:?}", data);
      }
      RecvMessage::TimeAndDate(_) => {
        tracing::trace!(target: "concord4::state::time-and-date", "unhandled: {:?}", data);
      }
      RecvMessage::LightsState(_) => {
        tracing::trace!(target: "concord4::state::lights-state", "unhandled: {:?}", data);
      }
      RecvMessage::UserLights(_) => {
        tracing::trace!(target: "concord4::state::user-lights", "unhandled: {:?}", data);
      }
      RecvMessage::Keyfob(_) => {
        tracing::trace!(target: "concord4::state::keyfob", "unhandled: {:?}", data);
      }
    };
  }

  fn handle_panel_type(&self, data: PanelData) {
    tracing::debug!(target: "concord4::state::panel-type", "setting panel: {:?}", data);

    let _ = self.panel.0.set(data);
  }

  fn handle_zone_data(&self, data: ZoneData) {
    tracing::debug!(target: "concord4::state::zone-data", "updating zone: {:?}", data);

    self.partitions.entry(data.partition_number).and_modify(|partition| {
      partition.zones.insert(data.id());
    });

    match self.groups.entry(data.group_id()) {
      dashmap::Entry::Occupied(mut group) => {
        group.get_mut().zones.insert(data.id());
      }
      dashmap::Entry::Vacant(group) => {
        group.insert(Group::new(data.partition_number, data.group_number));
      }
    }

    self.zones.insert(data.id(), data);
  }

  fn handle_zone_status(&self, data: ZoneStatusData) {
    tracing::debug!(target: "concord4::state::zone-status", "updating zone status: {:?}", data);

    self.zones.entry(data.zone_id()).and_modify(|zone| {
      zone.zone_status = data.zone_status;
    });
  }

  fn handle_partition_data(&self, mut data: PartitionData) {
    tracing::debug!(target: "concord4::state::partition-data", "updating partition: {:?}", data);

    data.zones = self
      .zones
      .iter()
      .filter_map(|zone| {
        if zone.partition_number == data.partition_number {
          Some(zone.id())
        } else {
          None
        }
      })
      .collect();

    self.partitions.insert(data.id(), data);
  }

  fn handle_arming_level(&self, data: ArmingLevelData) {
    tracing::debug!(target: "concord4::state::arming-level", "updating arming level: {:?}", data);

    self.partitions.entry(data.partition_number).and_modify(|partition| {
      partition.arming_level = data.arming_level;
    });
  }
}
