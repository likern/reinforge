use std::collections::VecDeque;

use crate::{
    arm::{ActionID, Reward},
    runner::run::Run,
};

#[derive(Debug, Clone, Copy)]
pub struct TimePoint<T> {
    pub step: usize,
    pub value: T,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RewardStats {
    count: usize,
    sum: f64,
}

impl RewardStats {
    pub fn update(&mut self, reward: Reward) {
        self.count += 1;
        self.sum += reward;
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
}

pub trait RunRewardStatsExt {
    fn average_rewards(&self) -> Vec<TimePoint<f64>>;

    fn cumulative_rewards(&self) -> Vec<TimePoint<f64>>;

    fn moving_average_rewards(&self, window_size: usize) -> Vec<TimePoint<f64>>;

    fn reward_stats_for_action(&self, action: ActionID) -> RewardStats;

    fn average_reward_for_action(&self, action: ActionID) -> Option<f64>;

    fn reward_stats_by_action(&self, arm_count: usize) -> Vec<RewardStats>;
}

impl RunRewardStatsExt for Run {
    fn average_rewards(&self) -> Vec<TimePoint<f64>> {
        self.iter()
            .enumerate()
            .scan(0.0, |sum, (idx, step)| {
                *sum += step.reward();

                Some(TimePoint {
                    step: idx + 1,
                    value: *sum / (idx + 1) as f64,
                })
            })
            .collect()
    }

    fn cumulative_rewards(&self) -> Vec<TimePoint<f64>> {
        self.iter()
            .enumerate()
            .scan(0.0, |sum, (idx, step)| {
                *sum += step.reward();

                Some(TimePoint {
                    step: idx + 1,
                    value: *sum,
                })
            })
            .collect()
    }

    fn moving_average_rewards(&self, window_size: usize) -> Vec<TimePoint<f64>> {
        assert!(
            window_size > 0,
            "window_size should be greater than zero because moving average needs a non-empty window"
        );

        let mut window = VecDeque::with_capacity(window_size);
        let mut sum = 0.0;

        self.iter()
            .enumerate()
            .map(|(idx, step)| {
                let reward = step.reward();

                window.push_back(reward);
                sum += reward;

                if window.len() > window_size {
                    let removed = window
                        .pop_front()
                        .expect("window should be non-empty after exceeding window_size");
                    sum -= removed;
                }

                TimePoint {
                    step: idx + 1,
                    value: sum / window.len() as f64,
                }
            })
            .collect()
    }

    fn reward_stats_for_action(&self, action: ActionID) -> RewardStats {
        self.iter().filter(|step| step.action() == action).fold(
            RewardStats::default(),
            |mut stats, step| {
                stats.update(step.reward());
                stats
            },
        )
    }

    fn average_reward_for_action(&self, action: ActionID) -> Option<f64> {
        self.reward_stats_for_action(action).mean()
    }

    fn reward_stats_by_action(&self, arm_count: usize) -> Vec<RewardStats> {
        let mut stats = vec![RewardStats::default(); arm_count];

        for step in self {
            let action = step.action();

            if action < arm_count {
                stats[action].update(step.reward());
            }
        }

        stats
    }
}
