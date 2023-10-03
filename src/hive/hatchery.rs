use std::{cell::RefCell, cmp::Ordering, collections::BinaryHeap, rc::Rc};

use crate::{
    creep_setup::CreepSetup, error::SwarmError, util::cast_room_object_into, zerg::CreepMemory,
};
use log::{debug, warn};
use screeps::{
    find::{Find, RoomObject},
    HasTypedId, Room, SpawnOptions, StructureSpawn,
};
use web_sys::console::warn;

pub struct Hatchery {
    spawns: Vec<StructureSpawn>,
    spawn_queue: BinaryHeap<SpawnRequests>,
    pub room: Room,
}

#[derive(Debug, Eq, PartialEq)]
struct SpawnRequests {
    priority: u32,
    setup: CreepSetup,
    overlord: String,
    colony: String,
}

impl Ord for SpawnRequests {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for SpawnRequests {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hatchery {
    pub fn new(room: &Room) -> Result<Self, SwarmError> {
        let spawns = room.find(RoomObject::MySpawns, None);

        if spawns.is_empty() {
            warn!("Hatchery::new. failed to find any spawn. {}", room.name());
            return Err(SwarmError::InternalAssertionFailed(
                "no valid spawn".to_string(),
            ));
        }

        Ok(Hatchery {
            spawns: spawns
                .into_iter()
                .map(|spawn| cast_room_object_into::<StructureSpawn>(spawn))
                .collect(),
            spawn_queue: BinaryHeap::new(),
            // todo(sheep): maybe use rc?
            room: room.clone(),
        })
    }

    pub fn request_for_spawn(self: &mut Self, setup: CreepSetup, overlord: String, priority: u32) {
        if self.room.energy_available() < setup.spawn_cost() {
            debug!("colony:request_for_spawn: ignore spawn request due to lack of energy avail: {}, need: {}.", self.room.energy_available(), setup.spawn_cost());
            return;
        }

        self.spawn_queue.push(SpawnRequests {
            priority: priority,
            setup: setup,
            overlord: overlord,
            colony: self.room.name().to_string(),
        })
    }

    pub fn run(&mut self) {
        for spawn in &self.spawns {
            Self::run_spawn(&self.room, &mut self.spawn_queue, spawn);
        }
    }

    fn run_spawn(room: &Room, binary_heap: &mut BinaryHeap<SpawnRequests>, spawn: &StructureSpawn) {
        if spawn.spawning().is_some() {
            return;
        }

        if binary_heap.is_empty() {
            return;
        }

        let spawn_request = binary_heap.peek();
        if spawn_request.is_none() {
            return;
        }
        let spawn_request = spawn_request.unwrap();

        if room.energy_available() < spawn_request.setup.spawn_cost() {
            debug!(
                "failed to spawn creep due to lack of energy. {:?} {:?}",
                spawn_request,
                room.energy_available()
            );
            return;
        }

        let body_pattern = &spawn_request.setup.pattern;
        let name = format!(
            "{}-{}-{}",
            room.name().to_string(),
            spawn_request.setup.role,
            spawn.id().to_u128()
        );
        let memory = CreepMemory {
            overlord: spawn_request.overlord.clone(),
            role: spawn_request.setup.role.clone(),
        }
        .into_value();
        let spawn_opts = SpawnOptions::new().memory(memory);

        let result = spawn.spawn_creep_with_options(&body_pattern, &name, &spawn_opts);
        if result.is_err() {
            warn!(
                "spawn creep failed. room name: {}, result: {:?}, spawn request: {:?}",
                room.name().to_string(),
                result.unwrap_err(),
                spawn_request
            );
        }
    }
}
