use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};

use crate::runner::trace_step::RunTrace;

fn format_f64_vec(values: &[f64]) -> String {
    let values = values
        .iter()
        .map(|value| format!("{value:.4}"))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{values}]")
}

fn format_usize_vec(values: &[usize]) -> String {
    let values = values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{values}]")
}

pub fn print_run_trace_table(trace: &RunTrace) {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            "t", "Q before", "N before", "scores", "greedy", "A", "reason", "R", "optimal",
            "Q after", "N after", "cum R", "avg R",
        ]);

    for step in trace.iter() {
        table.add_row(vec![
            Cell::new(step.t),
            Cell::new(format_f64_vec(&step.q_before)),
            Cell::new(format_usize_vec(&step.n_before)),
            Cell::new(format_f64_vec(&step.scores)),
            Cell::new(format!("{:?}", step.greedy_actions)),
            Cell::new(step.selected_action),
            Cell::new(step.reason.as_str()),
            Cell::new(format!("{:.4}", step.reward)),
            Cell::new(step.was_optimal),
            Cell::new(format_f64_vec(&step.q_after)),
            Cell::new(format_usize_vec(&step.n_after)),
            Cell::new(format!("{:.4}", step.cumulative_reward)),
            Cell::new(format!("{:.4}", step.running_average_reward)),
        ]);
    }

    println!("{table}");
}
