use screeps::ErrorCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwarmError {
    #[error("screep error {0:?}")]
    ScreepError(ErrorCode),
    #[error("internal assertion failed {0}")]
    InternalAssertionFailed(String),
}
