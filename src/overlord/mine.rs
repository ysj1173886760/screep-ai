use std::sync::Arc;
use screeps::{ErrorCode, ObjectId, Source};

use crate::{colony::Colony, error::SwarmError};

use super::{OverlordType, Overlord};

// one MineOverlord instance controls one source
struct MineOverlord {
  colony: Arc<Colony>,
  overlord_type: OverlordType,
  source: Source,
}

impl MineOverlord {
  fn new(colony: Arc<Colony>, source_id: ObjectId<Source>) -> Result<Arc<Self>, SwarmError> {
    todo!()
  }

  fn initialize_creeps() {

  }

  fn request_creeps() {

  }
}

impl Overlord for MineOverlord {
  fn run() -> Result<(), SwarmError> {
    Ok(())
  }

  fn get_name() -> String {
    todo!()
  }
}
