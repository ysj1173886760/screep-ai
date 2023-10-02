mod hatchery;

use std::rc::Rc;
pub use hatchery::Hatchery;
use screeps::{Room, ErrorCode, find::{Find, RoomObject}};

use crate::error::SwarmError;

struct Hive {
  hatcherys: Vec<Rc<Hatchery>>
}

impl Hive {
  fn new(room: &Room) -> Result<Hive, SwarmError> {
    todo!()
  }

  fn retrieve_spawns(room: &Room) -> Result<Hive, SwarmError> {
    let spawns = room.find(RoomObject::MySpawns, None);

    todo!()
  }
}