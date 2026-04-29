use rand::{Rng, RngExt};

use crate::{
    arm::ActionID,
    bandit::model::{ActionEstimate, ActionValueModel},
    policy::{
        epsilon::GreedyEvaluator,
        selection::{SelectionDecision, SelectionReason},
    },
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
        let greedy_eval = GreedyEvaluator::new(model);
        let partition = greedy_eval.partition();

        let greedy_decision = greedy_eval.random_greedy(&partition.greedy, rng);
        let action = greedy_decision.action;
        let reason = SelectionReason::Greedy(greedy_decision.reason);

        let q_values = model.q_values();
        SelectionDecision::new(action, q_values.to_vec(), partition.greedy, reason)
    }
}
