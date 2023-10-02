mod hatchery;

pub use hatchery::Hatchery;
use log::warn;
use screeps::{
    find::{Find, RoomObject},
    ErrorCode, Room, StructureSpawn,
};
use std::{cell::RefCell, rc::Rc};

use crate::{error::SwarmError, util::cast_room_object_into};

pub struct Hive {
    hatcherys: Hatchery,
}

impl Hive {
    pub fn new(room: &Room) -> Result<Rc<RefCell<Hive>>, SwarmError> {
        Ok(Rc::new(RefCell::new(Hive {
            hatcherys: Hatchery::new(room)?,
        })))
    }
}
