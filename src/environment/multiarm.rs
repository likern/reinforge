use std::collections::HashMap;

use rand::{Rng, RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    arm::{ActionID, ArmCount, ArmLike, ArmNorm, Reward},
    environment::Environment,
};

struct OptimalAction {
    action: ActionID,
    mean: f64,
}

pub struct MultiArmedEnv<R: Rng> {
    arms: HashMap<ActionID, ArmNorm<R>>,
    ids: Vec<ActionID>,
    optimal: OptimalAction,
}

impl MultiArmedEnv<ChaCha8Rng> {
    pub fn new(k: ArmCount, seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // Action with the highest mean reward so far
        let mut optimal: Option<OptimalAction> = None;

        let arms: HashMap<ActionID, ArmNorm<ChaCha8Rng>> = (0..k.get())
            .map(|idx| {
                let arm_rng_seed = rng.next_u64();
                let mut arm_rng = ChaCha8Rng::seed_from_u64(arm_rng_seed);

                let arm_mean: f64 = arm_rng.random();

                let is_higher = optimal.as_ref().map_or(true, |curr| arm_mean > curr.mean);
                if is_higher {
                    optimal = Some(OptimalAction {
                        action: idx,
                        mean: arm_mean,
                    });
                }

                let arm = ArmNorm::with_rng(arm_mean, 1.0, arm_rng);

                (idx, arm)
            })
            .collect();

        let optimal = optimal.expect("Optimal action not found, unexpected failure");

        let action_ids: Vec<ActionID> = arms.keys().map(|&key| key).collect();

        Self {
            arms: arms,
            ids: action_ids,
            optimal: optimal,
        }
    }

    pub fn action_ids(&self) -> &Vec<ActionID> {
        &self.ids
    }
}

impl Environment for MultiArmedEnv<ChaCha8Rng> {
    // Pull one arm, identified by action_id
    fn step(&mut self, action_id: ActionID) -> Reward {
        let selected_arm = self.arms.get_mut(&action_id).unwrap();
        let reward = selected_arm.pull();
        reward
    }

    fn is_optimal(&self, action_id: ActionID) -> bool {
        self.optimal.action == action_id
    }
}
