use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::{cell::RefCell, rc::Rc};

use log::*;
use screeps::{game, ErrorCode, FindConstant, HasTypedId, Room, RoomName};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::creep_setup::CreepSetup;
use crate::error::SwarmError;
use crate::hive::{self, Hive};
use crate::overlord::mine::MineOverlord;
use crate::overlord::{Overlord, OverlordType};

// according to https://docs.screeps.com/control.html
pub enum ColonyStage {
    Initialize,
    WithStorage,  // Level 4
    WithTerminal, // Level 6
    Mature,       // Level 8
}

pub struct Colony {
    pub rcl: u8,
    pub stage: ColonyStage,
    pub central_room_name: String,
    pub room: Room,
    pub hive: Rc<RefCell<Hive>>,
    pub overlords: HashMap<String, Box<dyn Overlord>>,
}

#[derive(Serialize, Deserialize)]
struct ColonyCache {
    mine_overlord_cache: Option<Vec<String>>,
}

impl Colony {
    pub fn new(
        central_room_name: String,
        cache: Option<JsValue>,
    ) -> Result<Rc<RefCell<Self>>, SwarmError> {
        let room_name: RoomName;
        match RoomName::new(&central_room_name) {
            Ok(res_room_name) => {
                room_name = res_room_name;
            }
            Err(e) => {
                warn!(
                    "Colony parse room name failed. {:?}, error: {:?}",
                    central_room_name, e
                );
                return Err(SwarmError::InternalAssertionFailed(
                    "parse colony room name failed.".to_string(),
                ));
            }
        }

        let room = game::rooms()
            .get(room_name)
            .ok_or(SwarmError::InternalAssertionFailed(
                "get room failed".to_string(),
            ))?;
        let rcl = room.controller().unwrap().level();

        let hive = Self::initialize_hive(&room)?;

        let colony_cache = if let Some(cache_value) = cache {
            let result_cache: Result<ColonyCache, _> = serde_wasm_bindgen::from_value(cache_value);
            if result_cache.is_err() {
                warn!(
                    "parse colony cache failed. {:?}",
                    result_cache.err().unwrap()
                );
                return Err(SwarmError::InternalAssertionFailed(
                    "parse colony cache failed".to_string(),
                ));
            }
            Some(result_cache.unwrap())
        } else {
            None
        };

        let overlords = Self::initialize_overlords(hive.clone(), &colony_cache)?;

        let colony = Colony {
            rcl: rcl,
            stage: Colony::get_colony_stage_by_rcl(rcl),
            central_room_name: central_room_name,
            room: room,
            hive: hive,
            overlords: overlords,
        };

        Ok(Rc::new(RefCell::new(colony)))
    }

    fn get_colony_stage_by_rcl(rcl: u8) -> ColonyStage {
        if rcl < 4 {
            return ColonyStage::Initialize;
        }
        if rcl < 6 {
            return ColonyStage::WithStorage;
        }
        if rcl < 8 {
            return ColonyStage::WithTerminal;
        }
        return ColonyStage::Mature;
    }

    fn initialize_hive(room: &Room) -> Result<Rc<RefCell<Hive>>, SwarmError> {
        Ok(Hive::new(room)?)
    }

    fn initialize_overlords(
        hive: Rc<RefCell<Hive>>,
        cache: &Option<ColonyCache>,
    ) -> Result<HashMap<String, Box<dyn Overlord>>, SwarmError> {
        // initialize mine overlord
        let mut overlord_map: HashMap<String, Box<dyn Overlord>> = HashMap::new();
        if cache.is_some() && cache.as_ref().unwrap().mine_overlord_cache.is_some() {
            Self::initialize_mine_overlord_by_cache(
                &mut overlord_map,
                hive,
                cache
                    .as_ref()
                    .unwrap()
                    .mine_overlord_cache
                    .as_ref()
                    .unwrap(),
            )?;
        } else {
            Self::initialize_mine_overlord(&mut overlord_map, hive)?;
        }

        Ok(overlord_map)
    }

    fn initialize_mine_overlord_by_cache(
        overlord_map: &mut HashMap<String, Box<dyn Overlord>>,
        hive: Rc<RefCell<Hive>>,
        cache: &Vec<String>,
    ) -> Result<(), SwarmError> {
        for overlord_cache in cache {
            let overlord = MineOverlord::new_from_cache(overlord_cache, hive.clone())?;
            let old_value = overlord_map.insert(overlord.get_name(), overlord);
            if old_value.is_some() {
                warn!("overlord has dup name: {}", old_value.unwrap().get_name())
            }
        }
        Ok(())
    }

    fn initialize_mine_overlord(
        overlord_map: &mut HashMap<String, Box<dyn Overlord>>,
        hive: Rc<RefCell<Hive>>,
    ) -> Result<(), SwarmError> {
        let sources = hive
            .as_ref()
            .borrow()
            .hatcherys
            .room
            .find(screeps::find::SOURCES_ACTIVE, None);
        for source in sources {
            let overlord = MineOverlord::new(source.id(), hive.clone())?;
            let old_value = overlord_map.insert(overlord.get_name(), overlord);
            if old_value.is_some() {
                warn!("overlord has dup name: {}", old_value.unwrap().get_name())
            }
        }
        Ok(())
    }
}
