use crate::arm::{ActionID, Reward};

mod multiarm;
pub use multiarm::MultiArmedEnv;

pub trait Environment {
    fn step(&mut self, action: ActionID) -> Reward;
    fn is_optimal(&self, action_id: ActionID) -> bool;
}
