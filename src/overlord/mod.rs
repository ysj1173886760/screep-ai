use screeps::{objects::Creep, ErrorCode};

use crate::{colony::Colony, error::SwarmError};
use std::sync::Arc;
use Creep as Zerg;

pub mod mine;

pub enum OverlordType {
    Mine,
}

pub trait Overlord {
    fn run(&self) -> Result<(), SwarmError>;

    // name of overlord must be globally unique.
    // [room_name]:[pos]:[overlord_name]
    fn get_name(&self) -> String;
}
