use log::warn;
use screeps::{game, Creep, ErrorCode, HasTypedId, ObjectId, Source};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen::{JsCast, JsValue};

use crate::{colony::Colony, error::SwarmError, hive::Hive, zerg::CreepMemory};

use super::{Overlord, OverlordType};

// one MineOverlord instance controls one source
pub struct MineOverlord {
    overlord_type: OverlordType,
    source: Source,
    hive: Rc<RefCell<Hive>>,
    creeps: Vec<Creep>,
}

#[derive(Serialize, Deserialize)]
struct MineOverlordCache {
    source_id: ObjectId<Source>,
    // todo(sheep): cache creeps
}

impl MineOverlord {
    pub fn new(
        source_id: ObjectId<Source>,
        hive: Rc<RefCell<Hive>>,
    ) -> Result<Box<Self>, SwarmError> {
        Self::new_internal(source_id, hive)
    }

    pub fn new_from_cache(
        cache: &String,
        hive: Rc<RefCell<Hive>>,
    ) -> Result<Box<Self>, SwarmError> {
        let overlord_cache = serde_json::from_str::<MineOverlordCache>(cache).map_err(|e| {
            warn!("Parse overlord cache failed");
            SwarmError::InternalAssertionFailed("Parse overlord cache failed".to_string())
        })?;
        Self::new_internal(overlord_cache.source_id, hive)
    }

    fn new_internal(
        source_id: ObjectId<Source>,
        hive: Rc<RefCell<Hive>>,
    ) -> Result<Box<Self>, SwarmError> {
        let creeps =
            Self::initialize_creeps(Self::get_name_internal(&hive.as_ref().borrow(), &source_id));
        let source = game::get_object_by_id_typed(&source_id).ok_or(
            SwarmError::InternalAssertionFailed("get source failed".to_string()),
        )?;

        Ok(Box::new(MineOverlord {
            overlord_type: OverlordType::Mine,
            source: source,
            hive: hive,
            creeps: creeps,
        }))
    }

    fn get_name_internal(hive: &Hive, source_id: &ObjectId<Source>) -> String {
        format!(
            "mine-{}-{}",
            hive.hatcherys.room.name().to_string(),
            source_id.to_u128()
        )
    }

    fn initialize_creeps(name: String) -> Vec<Creep> {
        game::creeps()
            .values()
            .filter(|creep| CreepMemory::from_value(creep.memory()).overlord == name)
            .collect()
    }
}

impl Overlord for MineOverlord {
    fn run(&self) -> Result<(), SwarmError> {
        Ok(())
    }

    fn get_name(&self) -> String {
        Self::get_name_internal(&self.hive.as_ref().borrow(), &self.source.id())
    }
}
