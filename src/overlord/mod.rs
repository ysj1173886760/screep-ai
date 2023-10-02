use screeps::{objects::Creep, ErrorCode};

use crate::{colony::Colony, error::SwarmError};
use std::sync::Arc;
use Creep as Zerg;

mod mine;

enum OverlordType {
    Mine,
}

trait Overlord {
    fn run() -> Result<(), SwarmError>;

    // name of overlord must be globally unique.
    // [room_name]:[pos]:[overlord_name]
    fn get_name() -> String;
}
