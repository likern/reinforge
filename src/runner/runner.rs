use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::{
    arm::ArmCount,
    bandit::{agent::BanditAgent, model::ActionReward},
    environment::{Environment, MultiArmedEnv},
    policy::policy::GreedyPolicy,
    runner::{
        Ensemble, RunnerBuildError,
        run::{Run, RunID},
        step::Step,
        trace::TraceMode,
        trace_step::RunTraceRecorder,
    },
};

#[derive(Debug, Clone)]
pub struct RunnerConfig {
    pub seed: u64,
    pub num_arms: usize,
    pub num_exps: usize,
    pub num_steps: usize,
    pub trace_mode: TraceMode,
}

pub struct Runner {
    seed: u64,
    arm_count: ArmCount,
    step_count: usize,
    run_count: usize,
    trace_mode: TraceMode,
}

impl Runner {
    pub fn try_new(config: RunnerConfig) -> Result<Self, RunnerBuildError> {
        let arm_count = ArmCount::non_trivial(config.num_arms)?;

        Ok(Self {
            arm_count,
            seed: config.seed,
            step_count: config.num_steps,
            run_count: config.num_exps,
            trace_mode: config.trace_mode,
        })
    }

    pub fn run_study(&self) -> Ensemble {
        let runs = (0..self.run_count)
            .map(|id| self.execute_single_run(id))
            .collect();

        Ensemble { runs }
    }

    pub fn execute_single_run(&self, id: RunID) -> Run {
        let policy = GreedyPolicy::new();
        let run_seed = self.seed + id as u64;

        let mut env = MultiArmedEnv::new(self.arm_count, run_seed);

        let policy_seed = run_seed ^ 0x9E37_79B9_7F4A_7C15;
        let mut policy_rng = ChaCha8Rng::seed_from_u64(policy_seed);

        let initial: Vec<ActionReward> = env
            .action_ids()
            .iter()
            .map(|action_id| ActionReward::new(*action_id, 0.0))
            .collect();

        let mut agent = BanditAgent::new(initial, policy);
        let mut run = Run::with_capacity(self.step_count);
        let mut trace = RunTraceRecorder::new(self.trace_mode, self.step_count);

        for step_idx in 0..self.step_count {
            let t = step_idx + 1;

            let before = trace.snapshot(|| agent.snapshot());

            let decision = agent.step(&mut policy_rng);
            let action = decision.action;

            let reward = env.step(action);
            let was_optimal = env.is_optimal(action);

            agent.observe(action, reward);

            let after = trace.snapshot(|| agent.snapshot());

            trace.record(t, before, &decision, reward, was_optimal, after);

            run.push(Step::new(action, reward, was_optimal));
        }

        run.set_trace(trace.finish());

        run
    }
}
