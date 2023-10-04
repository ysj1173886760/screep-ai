use log::{error, warn};
use screeps::{
    find, game, ConstructionSite, Creep, ErrorCode, HasPosition, HasTypedId, MoveToOptions,
    ObjectId, PolyStyle, Position, ResourceType, Room, SharedCreepProperties, Source,
    StructureController, StructureExtension, StructureObject, Transferable, TransferableObject,
};
use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen::{JsCast, JsValue};

use crate::{
    colony::Colony, creep_setup::creep_setup_templates::CreepSetupTemplate, error::SwarmError,
    hive::Hive, zerg::CreepMemory,
};

use super::{Overlord, OverlordType};

const MINER_MINING: &'static str = "mining";
const MINER_TRANSFERING: &'static str = "transfering";

// one MineOverlord instance controls one source
pub struct MineOverlord {
    overlord_type: OverlordType,
    source: Source,
    hive: Rc<RefCell<Hive>>,
    creeps: Vec<Creep>,
    room: Room,
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
        let room = hive.as_ref().borrow().hatcherys.room.clone();
        let creeps = Self::initialize_creeps(Self::get_name_internal(&room, &source_id));
        let source = game::get_object_by_id_typed(&source_id).ok_or(
            SwarmError::InternalAssertionFailed("get source failed".to_string()),
        )?;

        Ok(Box::new(MineOverlord {
            overlord_type: OverlordType::Mine,
            source: source,
            hive: hive,
            creeps: creeps,
            room: room,
        }))
    }

    fn get_name_internal(room: &Room, source_id: &ObjectId<Source>) -> String {
        format!("mine-{}-{}", room.name().to_string(), source_id.to_u128())
    }

    fn initialize_creeps(name: String) -> Vec<Creep> {
        game::creeps()
            .values()
            .filter(|creep| CreepMemory::from_value(creep.memory()).overlord == name)
            .collect()
    }

    fn maintain_creep(&self) {
        const miner_count: usize = 1;
        if self.creeps.len() >= miner_count {
            return;
        }

        let miner_needs_cnt = miner_count - self.creeps.len();
        for i in 0..miner_needs_cnt {
            self.hive.as_ref().borrow_mut().hatcherys.request_for_spawn(
                CreepSetupTemplate::drone(),
                self.get_name(),
                crate::constants::DEFAULT_PRIORITY,
            )
        }
    }

    fn run_miner(&self, creep: &Creep) -> Result<(), SwarmError> {
        let mut memory = CreepMemory::from_value(creep.memory());
        if memory.state.is_none() {
            memory.state = Some(MINER_TRANSFERING.to_string());
        }
        if creep.store().get_free_capacity(None) > 0
            && memory.state.as_ref().unwrap() == MINER_TRANSFERING
        {
            memory.state = Some(MINER_MINING.to_string())
        }
        if creep.store().get_free_capacity(None) == 0
            && memory.state.as_ref().unwrap() == MINER_MINING
        {
            memory.state = Some(MINER_TRANSFERING.to_string())
        }

        if memory.state.as_ref().unwrap() == MINER_TRANSFERING {
            self.run_miner_transfering(creep)?
        } else if memory.state.as_ref().unwrap() == MINER_MINING {
            self.run_miner_mining(creep)?
        } else {
            error!("invalid miner state {}", memory.state.as_ref().unwrap());
        }

        Ok(())
    }

    fn run_miner_transfering(&self, creep: &Creep) -> Result<(), SwarmError> {
        // try upgrade controller if it's going to downgrade
        let controller = self.room.controller();
        if controller.is_some() && controller.as_ref().unwrap().ticks_to_downgrade() < 200 {
            Self::do_upgrade(creep, controller.as_ref().unwrap());
            return Ok(());
        }

        // first try fill energy
        if Self::try_fill_energy(creep, &self.room) {
            return Ok(());
        }

        // then try build
        if Self::try_build(creep, &self.room) {
            return Ok(());
        }

        // todo(sheep): repair container

        // last, try upgrade
        if controller.is_some() {
            Self::do_upgrade(creep, controller.as_ref().unwrap());
            return Ok(());
        }

        Ok(())
    }

    fn run_miner_mining(&self, creep: &Creep) -> Result<(), SwarmError> {
        let res = creep.harvest(&self.source);
        if res.is_err() {
            if res.unwrap_err() == ErrorCode::NotInRange {
                Self::move_creep(self.source.pos(), creep);
            } else {
                warn!(
                    "overlord:run_miner_mining: unexpected error: {:?}",
                    res.unwrap_err()
                );
            }
        }
        Ok(())
    }

    fn try_fill_energy(creep: &Creep, room: &Room) -> bool {
        let targets = room
            .find(screeps::find::MY_STRUCTURES, None)
            .into_iter()
            .filter(|structure| {
                match structure {
                    StructureObject::StructureExtension(s) => {
                        return s.store().get_free_capacity(None) > 0
                    }
                    StructureObject::StructureSpawn(s) => {
                        return s.store().get_free_capacity(None) > 0
                    }
                    StructureObject::StructureTower(s) => {
                        return s.store().get_free_capacity(None) > 0
                    }
                    _ => return false,
                }
                unreachable!()
            })
            .next();

        if targets.is_some() {
            Self::do_transfer(targets.unwrap().as_transferable().unwrap(), creep);
            return true;
        }
        return false;
    }

    fn try_build(creep: &Creep, room: &Room) -> bool {
        let targets = room
            .find(screeps::find::MY_CONSTRUCTION_SITES, None)
            .into_iter()
            .next();
        if targets.is_some() {
            Self::do_build(targets.unwrap(), creep);
            return true;
        }
        return false;
    }

    fn move_creep(pos: Position, creep: &Creep) {
        let path_style = PolyStyle::default().stroke("#ffaa00'");
        let move_opts = MoveToOptions::default()
            .visualize_path_style(path_style)
            .reuse_path(10);
        let _ = creep.move_to_with_options(pos, Some(move_opts));
    }

    fn do_transfer(transferable_structure: &dyn Transferable, creep: &Creep) {
        let res = creep.transfer(transferable_structure, ResourceType::Energy, None);
        if res.is_err() {
            if res.unwrap_err() == ErrorCode::NotInRange {
                Self::move_creep(transferable_structure.pos(), creep);
            } else {
                warn!(
                    "overlord:do_transfer: unexpected error: {:?}",
                    res.unwrap_err()
                );
            }
        }
    }

    fn do_build(construction_site: ConstructionSite, creep: &Creep) {
        let res = creep.build(&construction_site);
        if res.is_err() {
            if res.unwrap_err() == ErrorCode::NotInRange {
                Self::move_creep(construction_site.pos(), creep);
            } else {
                warn!(
                    "overlord:do_build: unexpected error: {:?}",
                    res.unwrap_err()
                );
            }
        }
    }

    fn do_upgrade(creep: &Creep, controller: &StructureController) {
        let res = creep.upgrade_controller(controller);
        if res.is_err() {
            if res.unwrap_err() == ErrorCode::NotInRange {
                Self::move_creep(controller.pos(), creep);
            } else {
                warn!(
                    "overlord:do_upgrade: unexpected error: {:?}",
                    res.unwrap_err()
                );
            }
        }
    }
}

impl Overlord for MineOverlord {
    fn run(&self) -> Result<(), SwarmError> {
        self.maintain_creep();
        for creep in self.creeps.iter() {
            self.run_miner(creep)?
        }
        Ok(())
    }

    fn get_name(&self) -> String {
        Self::get_name_internal(&self.room, &self.source.id())
    }
}
