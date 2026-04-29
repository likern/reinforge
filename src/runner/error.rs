use crate::arm::ArmCountError;

#[derive(Debug, thiserror::Error)]
pub enum RunnerBuildError {
    #[error("invalid runner configuration: invalid arm count")]
    InvalidArmCount(#[from] ArmCountError),
}
