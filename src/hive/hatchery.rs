use screeps::StructureSpawn;

pub struct Hatchery {
  spawn: StructureSpawn,
}

impl Hatchery {
  pub fn new(spawn: StructureSpawn) -> std::rc::Rc<Self> {
    return std::rc::Rc::new(Hatchery { spawn: spawn })
  }
}