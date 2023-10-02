use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{cell::RefCell, rc::Rc};

use log::*;
use screeps::{game, ErrorCode, FindConstant, Room, RoomName};

use crate::creep_setup::CreepSetup;
use crate::error::SwarmError;
use crate::hive::{self, Hive};

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
}

impl Colony {
    pub fn new(central_room_name: String) -> Result<Rc<RefCell<Self>>, SwarmError> {
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

        let mut colony = Colony {
            rcl: rcl,
            stage: Colony::get_colony_stage_by_rcl(rcl),
            central_room_name: central_room_name,
            room: room,
            hive: hive,
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
}
