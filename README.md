# concord4-rs

A Rust library allowing access to the data provided by apcupsd.

## Installation

```sh
cargo add concord4
```

## Usage

```rust
use concord4::Concord4;

#[tokio::main]
async fn main() {
  let path = "/dev/ttyUSB0"; // path to the serial device for the alarm panel
  let concord = Concord4::init(path);

  concord.wait_ready().await; // waits for the state to initialize - takes about 30 seconds to get all data from panel

  let data = concord.data().await; // gets the data from the panel
  println!("data: {:?}", data);

  concord.block().await; // waits for a control-c to cleanup and exit
}
```

## Data Format

```rust
pub struct PublicState {
  pub panel: PanelData,
  pub zones: BTreeMap<String, ZoneData>,
  pub partitions: BTreeMap<String, PartitionData>,

  pub initialized: bool,
}
```

You can see the definition of the data format in the [state.rs](src/state.rs) and [equipment.rs](src/equipment.rs) files.

## Subscribe to Changes

```rust
pub enum StateData {
  Panel(PanelData),
  Zone(ZoneData),
  Partition(PartitionData),
}

let rx: tokio::sync::broadcast::Receiver<StateData> = concord.subscribe();

while let Ok(data) = rx.recv().await {
  match data {
    StateData::Panel(data) => {
      println!("panel data: {:?}", data);
    }
    StateData::Zone(data) => {
      println!("zone data: {:?}", data);
    }
    StateData::Partition(data) => {
      println!("partition data: {:?}", data);
    }
  }
}
```

This can be used to send partial data to a websocket, for example.

## Examples

See my project [concord4-ha](https://github.com/JoeyEamigh/concord4-ha) for a full example of how to use this library. That project is a Home Assistant integration for the Concord4 alarm panel, using websockets to stream updates.
