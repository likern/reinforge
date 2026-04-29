use crate::arm::{ActionID, Reward};

pub type ActionEstimate = f64;

#[derive(Debug, Clone, Copy)]
pub struct ActionReward {
    id: ActionID,
    reward: Reward,
}

impl ActionReward {
    pub fn new(id: ActionID, reward: Reward) -> Self {
        Self { id, reward }
    }
}

#[derive(Debug, Clone)]
pub struct ActionValueSnapshot {
    pub q_values: Vec<ActionEstimate>,
    pub action_counts: Vec<usize>,
    pub total_steps: usize,
}

pub struct ActionValueModel {
    q_values: Vec<ActionEstimate>,
    action_counts: Vec<usize>,
    total_steps: usize,
}

impl ActionValueModel {
    pub fn new(initial_estimates: Vec<ActionReward>) -> Self {
        let arm_count = initial_estimates.len();

        let mut q_values = vec![0.0; arm_count];

        for estimate in initial_estimates {
            assert!(
                estimate.id < arm_count,
                "action id should be less than arm_count because actions are stored densely by index"
            );

            q_values[estimate.id] = estimate.reward;
        }

        Self {
            q_values,
            action_counts: vec![0; arm_count],
            total_steps: 0,
        }
    }

    pub fn update(&mut self, action_id: ActionID, reward: Reward) {
        debug_assert!(
            action_id < self.q_values.len(),
            "action_id should be a valid dense action index"
        );

        self.total_steps += 1;

        let count = &mut self.action_counts[action_id];
        *count += 1;

        let n = *count as f64;
        let q = &mut self.q_values[action_id];

        *q += (reward - *q) / n;
    }

    pub fn q_values(&self) -> &[ActionEstimate] {
        &self.q_values
    }

    pub fn action_counts(&self) -> &[usize] {
        &self.action_counts
    }

    pub fn total_steps(&self) -> usize {
        self.total_steps
    }

    pub fn snapshot(&self) -> ActionValueSnapshot {
        ActionValueSnapshot {
            q_values: self.q_values.clone(),
            action_counts: self.action_counts.clone(),
            total_steps: self.total_steps,
        }
    }
}
