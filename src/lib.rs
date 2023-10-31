use communication::{RecvMessage, SendableMessage};
use equipment::{PanelData, PartitionData, ZoneData};
use futures::stream::{SplitSink, SplitStream, StreamExt};
use state::{ConcordState, StateData};
use std::{
  collections::BTreeMap,
  str,
  sync::{atomic::AtomicBool, Arc},
  time::Duration,
};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::codec::{Decoder, Framed};
use tracing::{debug, error, info, trace, warn};

use crate::communication::ListRequest;

mod communication;
mod consts;
mod decode;
mod equipment;
mod serial;
mod state;
mod touchpad;

#[derive(Debug, Clone)]
pub struct PublicState {
  pub panel: PanelData,
  pub zones: BTreeMap<String, ZoneData>,
  pub partitions: BTreeMap<String, PartitionData>,

  pub initialized: bool,
}

impl PublicState {
  fn new(
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

pub struct Concord4 {
  pub tx: tokio::sync::mpsc::Sender<SendableMessage>,

  state: ConcordState,

  reader_tx: tokio::sync::broadcast::Sender<StateData>,
  use_rx: Arc<AtomicBool>,
  ready_tx: tokio::sync::mpsc::Sender<consts::CtrlFlow>,

  _handles: Vec<tokio::task::JoinHandle<()>>,
}

impl Concord4 {
  pub fn init(path: &str) -> Self {
    use tokio_serial::SerialPort;

    let port = tokio_serial::new(path, consts::BAUD_RATE)
      .data_bits(consts::DATA_BITS)
      .parity(consts::PARITY)
      .timeout(Duration::from_millis(10))
      .open_native_async()
      .expect("Failed to open port");

    info!("Receiving data on {} at 9600 baud:", path);

    // must clear buffer because system won't know which message the ACKs are referring to
    port
      .clear(tokio_serial::ClearBuffer::All)
      .expect("Failed to clear buffer");

    let (writer_tx, writer_rx) = tokio::sync::mpsc::channel(100);
    let (ready_tx, ready_rx) = tokio::sync::mpsc::channel(10);
    let (reader_tx, _) = tokio::sync::broadcast::channel(10);

    let mut concord = Self {
      tx: writer_tx,
      state: ConcordState::new(),

      reader_tx: reader_tx.clone(),
      use_rx: Arc::new(AtomicBool::new(false)),
      ready_tx,

      _handles: vec![],
    };

    let (writer, reader) = concord.clone().framed(port).split();

    let reader_state = concord.clone();
    let writer_state = concord.clone();
    concord._handles.push(tokio::spawn(async move {
      reader_state.reader_listen(reader, reader_tx).await
    }));
    concord._handles.push(tokio::spawn(async move {
      writer_state.writer_listen(writer, writer_rx, ready_rx).await
    }));

    let data_init_tx = concord.tx.clone();
    concord._handles.push(tokio::spawn(async move {
      let _ = data_init_tx.try_send(SendableMessage::DynamicDataRefresh);
      tokio::time::sleep(std::time::Duration::from_secs(10)).await;
      let _ = data_init_tx.try_send(SendableMessage::List(ListRequest::AllData));
    }));

    concord
  }

  pub async fn wait_ready(&self) {
    while !self.state.initialized.load(std::sync::atomic::Ordering::Relaxed) {
      tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
  }

  pub async fn block(&self) {
    tokio::signal::ctrl_c().await.unwrap();
  }

  pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<StateData> {
    self.use_rx.store(true, std::sync::atomic::Ordering::Relaxed);

    self.reader_tx.subscribe()
  }

  pub async fn data(&self) -> PublicState {
    let panel = self.state.panel.read().await;
    let zones = self.state.zones.read().await;
    let partitions = self.state.partitions.read().await;

    PublicState::new(
      panel.to_owned(),
      zones.to_owned(),
      partitions.to_owned(),
      self.state.initialized.load(std::sync::atomic::Ordering::Relaxed),
    )
  }

  async fn reader_listen(
    &self,
    mut reader: SplitStream<Framed<SerialStream, Concord4>>,
    reader_tx: tokio::sync::broadcast::Sender<StateData>,
  ) {
    loop {
      // https://github.com/rust-lang/rust/issues/53667 - merge if let chains for the love of god
      if let Some(Ok(msg)) = reader.next().await {
        match msg.clone() {
          RecvMessage::PanelType(data) => {
            debug!("updating panel type: {:?}", data);

            let mut panel = self.state.panel.write().await;
            panel.panel_type = data.panel_type;

            self.send_to_reader_tx(&reader_tx, StateData::Panel(panel.to_owned()));
          }
          RecvMessage::ZoneData(data) => {
            debug!("updating zone: {:?}", data);

            let mut zones = self.state.zones.write().await;
            let zone_id = data.compute_id();
            zones.insert(zone_id.clone(), data);

            self.send_to_reader_tx(&reader_tx, StateData::Zone(zones.get(&zone_id).unwrap().to_owned()));
          }
          RecvMessage::ZoneStatus(data) => {
            debug!("updating zone status: {:?}", data);

            let mut zones = self.state.zones.write().await;

            let zone_id = data.compute_id();
            if let Some(zone) = zones.get_mut(&zone_id) {
              zone.zone_status = data.zone_status;

              self.send_to_reader_tx(&reader_tx, StateData::Zone(zones.get(&zone_id).unwrap().to_owned()));
            }
          }
          RecvMessage::PartitionData(data) => {
            debug!("updating partition: {:?}", data);

            let mut partitions = self.state.partitions.write().await;

            let partition_id = data.compute_id();
            partitions.insert(partition_id.clone(), data);

            self.send_to_reader_tx(
              &reader_tx,
              StateData::Partition(partitions.get(&partition_id).unwrap().to_owned()),
            );
          }
          RecvMessage::ArmingLevel(data) => {
            debug!("updating partition arming level: {:?}", data);

            let mut partitions = self.state.partitions.write().await;
            let partition_id = data.compute_id();

            if let Some(partition) = partitions.get_mut(&partition_id) {
              partition.arming_level = data.arming_level;

              self.send_to_reader_tx(
                &reader_tx,
                StateData::Partition(partitions.get(&partition_id).unwrap().to_owned()),
              );
            };
          }
          RecvMessage::EqptListDone => {
            if !self.state.initialized.load(std::sync::atomic::Ordering::Relaxed) {
              info!("state initialization complete - ready to go!");
              self.state.initialized.store(true, std::sync::atomic::Ordering::Relaxed);
            }
          }
          RecvMessage::SirenSync => {
            trace!(target: "concord4::siren-sync", "recv unhandled: {:?}", msg);
          }
          RecvMessage::Touchpad(_) => {
            trace!(target: "concord4::touchpad", "recv unhandled: {:?}", msg);
          }
          _ => {
            trace!("recv unhandled: {:?}", msg);
          }
        }
      }
    }
  }

  fn send_to_reader_tx(&self, reader_tx: &tokio::sync::broadcast::Sender<StateData>, msg: StateData) {
    if self.use_rx.load(std::sync::atomic::Ordering::Relaxed) && reader_tx.send(msg).is_err() {
      error!("failed to send message");
    }
  }

  async fn writer_listen(
    &self,
    mut writer: SplitSink<Framed<SerialStream, Concord4>, SendableMessage>,
    mut rx: tokio::sync::mpsc::Receiver<SendableMessage>,
    mut ready_rx: tokio::sync::mpsc::Receiver<consts::CtrlFlow>,
  ) {
    use futures::SinkExt;

    while let Some(line) = rx.recv().await {
      writer.send(line.clone()).await.unwrap();

      match line {
        SendableMessage::Ack => continue,
        SendableMessage::Nak => continue,
        _ => {
          // wait for ack before continuing
          if tokio::time::timeout(Duration::from_millis(2000), ready_rx.recv())
            .await
            .is_err()
          {
            warn!("timed out waiting for ack");
          }
        }
      }
    }
  }
}

impl Clone for Concord4 {
  fn clone(&self) -> Self {
    Self {
      tx: self.tx.clone(),
      state: self.state.clone(),

      reader_tx: self.reader_tx.clone(),
      use_rx: self.use_rx.clone(),
      ready_tx: self.ready_tx.clone(),

      _handles: vec![],
    }
  }
}

pub enum Error {
  Encoder,
  Decoder,
}