use screeps::Part;

use super::CreepSetup;

pub struct CreepSetupTemplate {}

impl CreepSetupTemplate {
    pub fn drone() -> CreepSetup {
        CreepSetup {
            role: "drone".to_string(),
            pattern: vec![Part::Work, Part::Work, Part::Move, Part::Carry],
        }
    }
}
