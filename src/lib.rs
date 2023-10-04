use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};
use std::rc::Rc;

use log::*;
use screeps::Room;
use screeps::{
    constants::{ErrorCode, Part, ResourceType},
    enums::StructureObject,
    find, game,
    local::ObjectId,
    objects::{Creep, Source, StructureController},
    prelude::*,
};
use wasm_bindgen::prelude::*;

use crate::colony::Colony;

mod colony;
mod constants;
mod creep_setup;
mod error;
mod hive;
mod logging;
mod memory;
mod overlord;
mod util;
mod zerg;

// add wasm_bindgen to any function you would like to expose for call from js
#[wasm_bindgen]
pub fn setup() {
    logging::setup_logging(logging::Debug);
}

fn get_room_with_spawn() -> Vec<Room> {
  let mut hash_map: HashMap<String, Room> = HashMap::new();
  game::spawns().values().for_each(|spawn| {
    hash_map.insert(spawn.room().unwrap().name().to_string(), spawn.room().unwrap());
  });
  hash_map.into_values().collect()
}

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    debug!("loop starting! CPU: {}", game::cpu::get_used());

    let rooms = get_room_with_spawn();
    let mut colonys:  Vec<Rc<RefCell<Colony>>> = Vec::new();
    for room in rooms {
      let colony = colony::Colony::new_from_room(room);
      if colony.is_err() {
        warn!("init colony failed. {:?}", colony.err().unwrap());
        continue;
      }
      colonys.push(colony.unwrap())
    }

    debug!("initialize colony done! cpu: {}", game::cpu::get_used());

    // run colonys
    for colony in colonys.iter() {
      colony.as_ref().borrow().run();
    }

    debug!("run colony done! cpu: {}", game::cpu::get_used());
}

// fn run_spawn() {
    // let mut additional = 0;
    // for spawn in game::spawns().values() {
    //     debug!("running spawn {}", String::from(spawn.name()));

    //     let body = [Part::Move, Part::Move, Part::Carry, Part::Work];
    //     if spawn.room().unwrap().energy_available() >= body.iter().map(|p| p.cost()).sum() {
    //         // create a unique name, spawn.
    //         let name_base = game::time();
    //         let name = format!("{}-{}", name_base, additional);
    //         // note that this bot has a fatal flaw; spawning a creep
    //         // creates Memory.creeps[creep_name] which will build up forever;
    //         // these memory entries should be prevented (todo doc link on how) or cleaned up
    //         match spawn.spawn_creep(&body, &name) {
    //             Ok(()) => additional += 1,
    //             Err(e) => warn!("couldn't spawn: {:?}", e),
    //         }
    //     }
    // }
// }

// fn run_creep(creep: &Creep, creep_targets: &mut HashMap<String, CreepTarget>) {
//     if creep.spawning() {
//         return;
//     }
//     let name = creep.name();
//     debug!("running creep {}", name);

//     let target = creep_targets.entry(name);
//     match target {
//         Entry::Occupied(entry) => {
//             let creep_target = entry.get();
//             match creep_target {
//                 CreepTarget::Upgrade(controller_id)
//                     if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 =>
//                 {
//                     if let Some(controller) = controller_id.resolve() {
//                         creep
//                             .upgrade_controller(&controller)
//                             .unwrap_or_else(|e| match e {
//                                 ErrorCode::NotInRange => {
//                                     let _ = creep.move_to(&controller);
//                                 }
//                                 _ => {
//                                     warn!("couldn't upgrade: {:?}", e);
//                                     entry.remove();
//                                 }
//                             });
//                     } else {
//                         entry.remove();
//                     }
//                 }
//                 CreepTarget::Harvest(source_id)
//                     if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 =>
//                 {
//                     if let Some(source) = source_id.resolve() {
//                         if creep.pos().is_near_to(source.pos()) {
//                             creep.harvest(&source).unwrap_or_else(|e| {
//                                 warn!("couldn't harvest: {:?}", e);
//                                 entry.remove();
//                             });
//                         } else {
//                             let _ = creep.move_to(&source);
//                         }
//                     } else {
//                         entry.remove();
//                     }
//                 }
//                 _ => {
//                     entry.remove();
//                 }
//             };
//         }
//         Entry::Vacant(entry) => {
//             // no target, let's find one depending on if we have energy
//             let room = creep.room().expect("couldn't resolve creep room");
//             if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
//                 for structure in room.find(find::STRUCTURES, None).iter() {
//                     if let StructureObject::StructureController(controller) = structure {
//                         entry.insert(CreepTarget::Upgrade(controller.id()));
//                         break;
//                     }
//                 }
//             } else if let Some(source) = room.find(find::SOURCES_ACTIVE, None).get(0) {
//                 entry.insert(CreepTarget::Harvest(source.id()));
//             }
//         }
//     }
// }
