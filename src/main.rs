mod arm;
mod bandit;
mod environment;
mod plot;
mod policy;
mod runner;
mod statistics;

use crate::runner::{Runner, RunnerConfig, TraceMode, print_run_trace_table};

use crate::statistics::prelude::*;

const K: usize = 3;
const COUNT: usize = 10;
const SEED: u64 = 0x50FFC001;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RunnerConfig {
        seed: SEED,
        num_arms: K,
        num_steps: COUNT,
        num_exps: 1,
        trace_mode: TraceMode::On,
    };

    let runner = Runner::try_new(config)?;
    let ensemble = runner.run_study();
    let first_run = &ensemble.runs[0];

    if let Some(trace) = first_run.trace() {
        print_run_trace_table(trace);
    }

    let avg_rewards = first_run.average_rewards();
    println!("{:#?}", avg_rewards);

    println!("Done.");
    Ok(())
}
