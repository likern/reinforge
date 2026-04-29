use rand::{Rng, RngExt};

use crate::{
    arm::ActionID,
    bandit::model::{ActionEstimate, ActionValueModel},
    policy::selection::{SelectionDecision, SelectionReason},
};

const EPS: f64 = 1e-12;

pub trait Policy {
    fn select_action<R: Rng + ?Sized>(
        &mut self,
        model: &ActionValueModel,
        rng: &mut R,
    ) -> SelectionDecision;
}

pub struct GreedyPolicy {}

impl GreedyPolicy {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GreedyPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl Policy for GreedyPolicy {
    fn select_action<R: Rng + ?Sized>(
        &mut self,
        model: &ActionValueModel,
        rng: &mut R,
    ) -> SelectionDecision {
        let q_values = model.q_values();

        let max_estimate = q_values
            .iter()
            .copied()
            .reduce(f64::max)
            .expect("ActionValueModel should contain at least one q-value");

        let greedy_actions: Vec<ActionID> = q_values
            .iter()
            .enumerate()
            .filter_map(|(action_id, estimate)| {
                let diff = (*estimate - max_estimate).abs();

                if diff <= EPS { Some(action_id) } else { None }
            })
            .collect();

        debug_assert!(
            !greedy_actions.is_empty(),
            "there should be at least one greedy action"
        );

        let idx = rng.random_range(0..greedy_actions.len());
        let action = greedy_actions[idx];

        let reason = if greedy_actions.len() == 1 {
            SelectionReason::Greedy
        } else {
            SelectionReason::TieBreak
        };

        SelectionDecision::new(action, q_values.to_vec(), greedy_actions, reason)
    }
}
