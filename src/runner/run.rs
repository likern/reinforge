use crate::{
    arm::{ActionID, Reward},
    runner::{step::Step, trace_step::RunTrace},
};

pub type RunID = usize;

#[derive(Debug, Default, Clone)]
pub struct Run {
    steps: Vec<Step>,
    trace: Option<RunTrace>,
}

impl Run {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            steps: Vec::with_capacity(capacity),
            trace: None,
        }
    }

    pub fn push(&mut self, step: Step) {
        self.steps.push(step);
    }

    pub fn set_trace(&mut self, trace: Option<RunTrace>) {
        self.trace = trace;
    }

    pub fn trace(&self) -> Option<&RunTrace> {
        self.trace.as_ref()
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn steps_mut(&mut self) -> &mut [Step] {
        &mut self.steps
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Step> {
        self.steps.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Step> {
        self.steps.iter_mut()
    }

    pub fn into_steps(self) -> Vec<Step> {
        self.steps
    }

    pub fn steps_by_action(&self, action: ActionID) -> impl Iterator<Item = &Step> {
        self.steps
            .iter()
            .filter(move |step| step.action() == action)
    }

    pub fn rewards_by_action(&self, action: ActionID) -> impl Iterator<Item = Reward> + '_ {
        self.steps_by_action(action).map(Step::reward)
    }

    pub fn average_reward_for_action(&self, action: ActionID) -> Option<f64> {
        let (sum, count) = self
            .rewards_by_action(action)
            .fold((0.0, 0usize), |(sum, count), reward| {
                (sum + reward, count + 1)
            });

        if count == 0 {
            None
        } else {
            Some(sum / count as f64)
        }
    }
}

impl<'a> IntoIterator for &'a Run {
    type Item = &'a Step;
    type IntoIter = std::slice::Iter<'a, Step>;

    fn into_iter(self) -> Self::IntoIter {
        self.steps.iter()
    }
}
