use std::{cell::RefCell, cmp::Ordering, collections::BinaryHeap, rc::Rc};

use crate::{creep_setup::CreepSetup, error::SwarmError, util::cast_room_object_into};
use log::{debug, warn};
use screeps::{
    find::{Find, RoomObject},
    Room, StructureSpawn,
};

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
        todo!("handle spawning requests");
    }
}
