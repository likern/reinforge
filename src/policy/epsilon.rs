use rand::Rng;
use rand::RngExt;

use crate::bandit::model::ActionEstimate;
use crate::policy::selection::GreedyDecision;
use crate::policy::selection::GreedyReason;
use crate::{
    arm::ActionID,
    bandit::model::ActionValueModel,
    policy::{
        policy::Policy,
        selection::{SelectionDecision, SelectionReason},
    },
};

const EPS: f64 = 1e-12;

pub struct EpsilonGreedyPolicy {
    epsilon: f64,
}

impl EpsilonGreedyPolicy {
    pub fn new(epsilon: f64) -> Self {
        Self { epsilon }
    }
}

pub type GreedyActionID = ActionID;
pub type EpsilonActionID = ActionID;

pub struct Partition {
    pub greedy: Vec<GreedyActionID>,
    pub epsilon: Vec<EpsilonActionID>,
}

pub struct GreedyEvaluator<'a> {
    model: &'a ActionValueModel,
}

impl<'a> GreedyEvaluator<'a> {
    pub fn new(model: &'a ActionValueModel) -> Self {
        Self { model }
    }

    pub fn max(&self) -> ActionEstimate {
        let q_values = self.model.q_values();
        let max_q_val = q_values
            .iter()
            .copied()
            .reduce(f64::max)
            .expect("ActionValueModel should contain at least one q-value");
        max_q_val
    }

    pub fn partition(&self) -> Partition {
        let max = self.max();
        let q_values = self.model.q_values();

        let is_greedy = |id: &ActionID| {
            let q_value = q_values[*id];
            q_value == max
        };

        let action_counts = self.model.action_counts();
        let action_range = 0..action_counts.len();

        let (greedy_ids, epsilon_ids): (Vec<ActionID>, Vec<ActionID>) =
            action_range.partition(is_greedy);

        Partition {
            greedy: greedy_ids,
            epsilon: epsilon_ids,
        }
    }

    pub fn random_greedy<R: Rng + ?Sized>(
        &self,
        greedy_ids: &[ActionID],
        rng: &mut R,
    ) -> GreedyDecision {
        debug_assert!(
            !greedy_ids.is_empty(),
            "greedy_ids should be non-empty because greedy action selection requires at least one candidate"
        );
        let idx = rng.random_range(0..greedy_ids.len());
        let action = greedy_ids[idx];

        let reason = if greedy_ids.len() == 1 {
            GreedyReason::RandomTieBreak
        } else {
            GreedyReason::UniqueBest
        };

        GreedyDecision { action, reason }
    }
}

impl Policy for EpsilonGreedyPolicy {
    fn select_action<R: Rng + ?Sized>(
        &mut self,
        model: &ActionValueModel,
        rng: &mut R,
    ) -> SelectionDecision {
        let greedy_eval = GreedyEvaluator::new(model);
        let partition = greedy_eval.partition();

        let epsilon = self.epsilon;

        let random_choice = if self.epsilon == 0.0f64 {
            // With epsilon = 0, epsilon-greedy degenerates to pure greedy selection.
            // Skip the exploration RNG draw so this path matches GreedyPolicy exactly.
            None
        } else {
            let random_choice = rng.random_range(0.0..1.0);
            Some(random_choice)
        };

        let (action, reason) = if let Some(random_choice) = random_choice && random_choice < epsilon {
            // Select a random action except greedy actions
            let epsilon_actions = partition.epsilon;
            let idx = rng.random_range(0..epsilon_actions.len());
            let action = epsilon_actions[idx];
            let reason = SelectionReason::EpsilonExplore;
            (action, reason)
        } else {
            // Select a greedy action
            let greedy_decision = greedy_eval.random_greedy(&partition.greedy, rng);
            let action = greedy_decision.action;
            let reason = SelectionReason::Greedy(greedy_decision.reason);
            (action, reason)
        };

        let q_values = model.q_values();
        SelectionDecision::new(action, q_values.to_vec(), partition.greedy, reason)
    }
}
