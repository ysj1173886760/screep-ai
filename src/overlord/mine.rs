use screeps::{game, ErrorCode, ObjectId, Source};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasm_bindgen::{JsCast, JsValue};

use crate::{colony::Colony, error::SwarmError};

use super::{Overlord, OverlordType};

// one MineOverlord instance controls one source
pub struct MineOverlord {
    overlord_type: OverlordType,
    source: Source,
}

#[derive(Serialize, Deserialize)]
pub struct MineOverlordCache {
    source_id: ObjectId<Source>,
}

impl MineOverlord {
    pub fn new(source_id: ObjectId<Source>) -> Result<Box<Self>, SwarmError> {
        todo!()
    }

    pub fn new_from_cache(cache: JsValue) -> Result<Box<Self>, SwarmError> {
        todo!()
    }

    fn initialize_creeps() {}

    fn request_creeps() {}
}

impl Overlord for MineOverlord {
    fn run(&self) -> Result<(), SwarmError> {
        Ok(())
    }

    fn get_name(&self) -> String {
        todo!()
    }
}
