use std::{
  collections::BTreeMap,
  sync::{atomic::AtomicBool, Arc},
};
use tokio::sync::RwLock;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use crate::equipment::{PanelData, PartitionData, ZoneData};

#[derive(Debug, Clone)]
pub struct ConcordState {
  pub panel: Arc<RwLock<PanelData>>,
  pub zones: Arc<RwLock<BTreeMap<String, ZoneData>>>,
  pub partitions: Arc<RwLock<BTreeMap<String, PartitionData>>>,

  pub initialized: Arc<AtomicBool>,
}

impl ConcordState {
  pub fn new() -> Self {
    Self {
      panel: Arc::new(RwLock::new(PanelData::default())),
      zones: Arc::new(RwLock::new(BTreeMap::new())),
      partitions: Arc::new(RwLock::new(BTreeMap::new())),

      initialized: Arc::new(AtomicBool::new(false)),
    }
  }
}

impl Default for ConcordState {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub enum StateData {
  Panel(PanelData),
  Zone(ZoneData),
  Partition(PartitionData),
}

#[cfg(feature = "json")]
impl StateData {
  pub fn to_json(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string(&self)
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct PublicState {
  pub panel: PanelData,
  pub zones: BTreeMap<String, ZoneData>,
  pub partitions: BTreeMap<String, PartitionData>,

  pub initialized: bool,
}

impl PublicState {
  pub fn new(
    panel: PanelData,
    zones: BTreeMap<String, ZoneData>,
    partitions: BTreeMap<String, PartitionData>,
    initialized: bool,
  ) -> Self {
    Self {
      panel,
      zones,
      partitions,

      initialized,
    }
  }
}

#[cfg(feature = "json")]
impl PublicState {
  pub fn to_json(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string(&self)
  }
}
