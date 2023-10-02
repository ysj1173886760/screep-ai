use screeps::Part;

pub mod creep_setup_templates;

#[derive(Debug, PartialEq, Eq)]
pub struct CreepSetup {
    role: String,
    pattern: Vec<Part>,
}

impl CreepSetup {
    pub fn spawn_cost(&self) -> u32 {
        self.pattern.iter().map(|part| Part::cost(*part)).sum()
    }
}
