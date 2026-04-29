use crate::arm::{ActionID, Reward};

#[derive(Debug, Clone)]
pub struct Step {
    action: ActionID,
    reward: Reward,
    was_optimal: bool,
}

impl Step {
    pub fn new(action: ActionID, reward: Reward, was_optimal: bool) -> Self {
        Self {
            action,
            reward,
            was_optimal,
        }
    }

    pub fn action(&self) -> ActionID {
        self.action
    }

    pub fn reward(&self) -> Reward {
        self.reward
    }

    pub fn was_optimal(&self) -> bool {
        self.was_optimal
    }
}
