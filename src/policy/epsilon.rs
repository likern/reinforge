use rand::Rng;
use rand::RngExt;

use crate::bandit::model::ActionEstimate;
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

pub struct GreedyAction {
    pub max: ActionEstimate,
    pub ids: Vec<ActionID>,
}

pub struct EpsilonAction {
    pub ids: Vec<ActionID>,
}

pub struct Partition {
    pub greedy: GreedyAction,
    pub epsilon: EpsilonAction,
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

        // let is_greedy = |id: &ActionID| {
        //     let q_value = q_values[*id];
        //     let diff = (q_value - max).abs();
        //     diff <= EPS
        // };

        let is_greedy = |id: &ActionID| {
            let q_value = q_values[*id];
            q_value == max
        };

        let action_counts = self.model.action_counts();
        let action_range = 0..action_counts.len();

        let (greedy_ids, epsilon_ids): (Vec<ActionID>, Vec<ActionID>) =
            action_range.partition(is_greedy);

        Partition {
            greedy: GreedyAction {
                max: max,
                ids: greedy_ids,
            },
            epsilon: EpsilonAction { ids: epsilon_ids },
        }
    }
}

impl Policy for EpsilonGreedyPolicy {
    fn select_action<R: Rng + ?Sized>(
        &mut self,
        model: &ActionValueModel,
        rng: &mut R,
    ) -> SelectionDecision {
        let q_values = model.q_values();
        let epsilon = self.epsilon;

        let max_estimate = q_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);

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

        let action_counts = model.action_counts();
        let action_range = (0..action_counts.len());

        let greedy_eval = GreedyEvaluator::new(model);
        let partition = greedy_eval.partition();

        let (greedy_actions, epsilon_actions): (Vec<ActionID>, Vec<ActionID>) =
            action_range.partition(is_greedy);

        // let (greedy_actions, epsilon_actions) = q_values
        //     .iter()
        //     .enumerate()
        //     .partition(|(action_id, estimate)| {

        //     });

        let random_choice = rng.random_range(0.0..1.0);

        if random_choice < epsilon {
            // Select a random action except greedy actions
        }

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
