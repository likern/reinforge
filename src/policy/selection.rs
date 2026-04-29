use crate::{arm::ActionID, bandit::model::ActionEstimate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionReason {
    Greedy,
    TieBreak,
    EpsilonExplore,
    Ucb,
}

impl SelectionReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Greedy => "greedy",
            Self::TieBreak => "tie_break",
            Self::EpsilonExplore => "epsilon_explore",
            Self::Ucb => "ucb",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectionDecision {
    pub action: ActionID,
    pub scores: Vec<ActionEstimate>,
    pub greedy_actions: Vec<ActionID>,
    pub reason: SelectionReason,
}

impl SelectionDecision {
    pub fn new(
        action: ActionID,
        scores: Vec<ActionEstimate>,
        greedy_actions: Vec<ActionID>,
        reason: SelectionReason,
    ) -> Self {
        Self {
            action,
            scores,
            greedy_actions,
            reason,
        }
    }
}
