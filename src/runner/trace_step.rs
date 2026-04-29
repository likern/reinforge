use crate::{
    arm::{ActionID, Reward},
    bandit::model::{ActionEstimate, ActionValueSnapshot},
    policy::selection::{SelectionDecision, SelectionReason},
    runner::trace::TraceMode,
};

#[derive(Debug, Clone)]
pub struct TraceStep {
    pub t: usize,

    pub q_before: Vec<ActionEstimate>,
    pub n_before: Vec<usize>,

    pub scores: Vec<ActionEstimate>,
    pub greedy_actions: Vec<ActionID>,

    pub selected_action: ActionID,
    pub reason: SelectionReason,

    pub reward: Reward,
    pub was_optimal: bool,

    pub q_after: Vec<ActionEstimate>,
    pub n_after: Vec<usize>,

    pub cumulative_reward: f64,
    pub running_average_reward: f64,
}

#[derive(Debug, Default, Clone)]
pub struct RunTrace {
    steps: Vec<TraceStep>,
}

impl RunTrace {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            steps: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, step: TraceStep) {
        self.steps.push(step);
    }

    pub fn steps(&self) -> &[TraceStep] {
        &self.steps
    }

    pub fn iter(&self) -> std::slice::Iter<'_, TraceStep> {
        self.steps.iter()
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

#[derive(Debug)]
pub struct RunTraceRecorder {
    trace: Option<RunTrace>,
    cumulative_reward: f64,
}

impl RunTraceRecorder {
    pub fn new(mode: TraceMode, capacity: usize) -> Self {
        let trace = match mode {
            TraceMode::Off => None,
            TraceMode::On => Some(RunTrace::with_capacity(capacity)),
        };

        Self {
            trace,
            cumulative_reward: 0.0,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.trace.is_some()
    }

    pub fn snapshot<F>(&self, capture: F) -> Option<ActionValueSnapshot>
    where
        F: FnOnce() -> ActionValueSnapshot,
    {
        if self.is_enabled() {
            Some(capture())
        } else {
            None
        }
    }

    pub fn record(
        &mut self,
        t: usize,
        before: Option<ActionValueSnapshot>,
        decision: &SelectionDecision,
        reward: Reward,
        was_optimal: bool,
        after: Option<ActionValueSnapshot>,
    ) {
        self.cumulative_reward += reward;

        let Some(trace) = &mut self.trace else {
            return;
        };

        let before =
            before.expect("trace before-snapshot should be present because TraceMode is enabled");
        let after =
            after.expect("trace after-snapshot should be present because TraceMode is enabled");

        trace.push(TraceStep {
            t,

            q_before: before.q_values,
            n_before: before.action_counts,

            scores: decision.scores.clone(),
            greedy_actions: decision.greedy_actions.clone(),

            selected_action: decision.action,
            reason: decision.reason,

            reward,
            was_optimal,

            q_after: after.q_values,
            n_after: after.action_counts,

            cumulative_reward: self.cumulative_reward,
            running_average_reward: self.cumulative_reward / t as f64,
        });
    }

    pub fn finish(self) -> Option<RunTrace> {
        self.trace
    }
}
