use rand::Rng;

use crate::{
    arm::{ActionID, Reward},
    bandit::model::{ActionReward, ActionValueModel, ActionValueSnapshot},
    policy::{policy::Policy, selection::SelectionDecision},
};

pub struct BanditAgent<P: Policy> {
    model: ActionValueModel,
    policy: P,
}

impl<P: Policy> BanditAgent<P> {
    pub fn new(initial_rewards: Vec<ActionReward>, policy: P) -> Self {
        let model = ActionValueModel::new(initial_rewards);
        Self { model, policy }
    }

    pub fn step<R: Rng + ?Sized>(&mut self, rng: &mut R) -> SelectionDecision {
        self.policy.select_action(&self.model, rng)
    }

    pub fn observe(&mut self, action_id: ActionID, reward: Reward) {
        self.model.update(action_id, reward);
    }

    pub fn snapshot(&self) -> ActionValueSnapshot {
        self.model.snapshot()
    }
}
