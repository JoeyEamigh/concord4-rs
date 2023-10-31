use std::{
  collections::BTreeMap,
  sync::{atomic::AtomicBool, Arc},
};

use tokio::sync::RwLock;

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
pub enum StateData {
  Panel(PanelData),
  Zone(ZoneData),
  Partition(PartitionData),
}
