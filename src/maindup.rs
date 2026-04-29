use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};
use std::f64;

mod arm;
mod bandit;
mod environment;
mod plot;
mod runner;
mod statistics;

use plotly::{ImageFormat, Plot};

use rand::prelude::*;
use rand_distr::StandardNormal;

use crate::{
    arm::ArmNorm,
    plot::violin::{
        enums::{ViolinMode, ViolinPoints},
        layout::{ViolinLayoutExtras, write_image_with_layout_extras},
        trace::Violin,
    },
    runner::{Runner, RunnerConfig, run::Run},
};

use crate::statistics::prelude::*;

// const K: usize = 10;
// const COUNT: usize = 100_000;
// const SEED: u64 = 0x50FFC001;
const K: usize = 3;
const COUNT: usize = 10;
const SEED: u64 = 0x50FFC001;

fn gen_normal_rand_vals(num: usize) -> Vec<f64> {
    let norm_distr = rand::rng().sample_iter::<f64, _>(StandardNormal);
    // let rand_vals = norm_distr.enumerate().take(num).collect();
    let rand_vals = norm_distr.take(num).collect();
    rand_vals
}

// pub fn calc_average_rewards(run: &Run) -> Vec<f64> {
//     let mut rewards: Vec<f64> = vec![0.0; run.steps.len()];
//     for (i, step) in run.steps.iter().enumerate() {
//         rewards[i] = step.reward;
//     }
//     rewards
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RunnerConfig {
        seed: SEED,
        num_arms: K,
        num_steps: COUNT,
        num_exps: 1,
        trace_mode: runner::trace::TraceMode::Off,
    };

    let mut runner = Runner::try_new(config)?;
    let ensemble = runner.run_study();
    let first_run = &ensemble.runs[0];

    {
        let mut table = Table::new();
        let cell = Cell::new("1");
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["Step", "Action", "Reward"]);

        for (idx, step) in first_run.iter().enumerate() {
            table.add_row(vec![
                Cell::new(idx),
                Cell::new(step.action()),
                Cell::new(step.reward()),
            ]);
        }

        println!("{table}");
    }

    {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["Step", "Action", "Reward"]);

        let avg_rewards = first_run.average_rewards();
        // println!("{:?}", avg_rewards);

        for timepoint in avg_rewards.iter() {
            table.add_row(vec![Cell::new(timepoint.step), Cell::new(timepoint.value)]);
        }

        println!("{table}");
    }

    // table.add_row(vec![cell, Cell::new(12)]);

    // let reward_history = runner.history.dump();
    // println!("reward_history:");
    // println!("{:?}", ensemble);

    // let k_rand_vals = gen_normal_rand_vals(num_arms);
    // let arms: Vec<ArmNorm> = k_rand_vals
    //     .iter()
    //     .map(|mean| ArmNorm::new(*mean, 1.0))
    //     .collect();

    // Run main cycle
    // Run greedy strategy for
    //
    // START !!!!!

    // let mut plot = Plot::new();

    // let y1 = gen_normal_rand_vals(10_000);
    // let y2 = gen_normal_rand_vals(10_000);

    // let x1 = vec!["A"; y1.len()];
    // let x2 = vec!["A"; y2.len()];

    // plot.add_trace(
    //     Violin::new_xy(x1, y1)
    //         .name("trace 1")
    //         .points(ViolinPoints::False)
    //         .box_visible(true)
    //         .meanline_visible(true),
    // );

    // plot.add_trace(
    //     Violin::new_xy(x2, y2)
    //         .name("trace 2")
    //         .points(ViolinPoints::False)
    //         .box_visible(true)
    //         .meanline_visible(true),
    // );

    // let extras = ViolinLayoutExtras::new().violin_mode(ViolinMode::Group);
    // write_image_with_layout_extras(
    //     &plot,
    //     "multi-violin.svg",
    //     ImageFormat::SVG,
    //     800,
    //     600,
    //     1.0,
    //     &extras,
    // )?;

    // plot.write_image("multi_violin.svg", ImageFormat::SVG, 800, 600, 1.0)?;

    //////// END !!!!!!

    // println!("Arms: {:#?}", arms);

    // let mut plot = Plot::new();
    //let hist_1 = Violin::new(gen_normal_rand_vals(10));
    // plot.add_trace(hist_1);

    // let hist_2 = Histogram::new(gen_normal_rand_vals(10));
    // plot.add_trace(hist_2);
    // plot.add_trace(Scatter::new(vec![0, 1, 2], vec![2, 1, 0]));

    // plot.write_image("test_out.svg", ImageFormat::SVG, 800, 600, 1.0)?;

    // let x: Vec<usize> = vals.iter().map(|x| x.0).collect();
    // let y = vals.iter().map(|x| x.1).collect();

    // let mut df: DataFrame = df!("name" => ["x", "y"]);
    // let v = ColumnVector::from(vec![10, 20, 30]);
    // let v = ColumnVector::from(&values1);

    // let dataset = Dataset::new()
    //     .with_column("x", v.clone())?
    //     .with_column("y", v.clone())?;

    // let df = DataFrame::empty();
    // // let ds = load_polars_df!(df)?;
    // let x_axis = X::new("x").with_bins(100);
    // let y_axis = Y::new("y");
    // // let color1 = Color::new("cyl");
    // let chart1 = Chart::build(dataset.clone())?
    //     .mark_hist()?
    //     .encode((x_axis.clone(), y_axis.clone()))?;

    // let norm_distr = rand::rng().sample_iter::<f64, _>(StandardNormal);
    // let values2: Vec<f64> = norm_distr.take(count).collect();
    // let column_vector2 = ColumnVector::from(&values2);

    // let dataset2 = Dataset::new()
    //     .with_column("x", &values2)?
    //     .with_column("y", &values2)?;

    // let chart2 = Chart::build(dataset2)?
    //     .mark_hist()?
    //     .encode((x_axis.clone(), y_axis.clone()))?;

    // chart1.and(chart2).save("out_test.svg")?;
    // .save("out_test.svg")?;
    // let ds = Dataset::new().with_column("x", x)?.with_column("y", y)?;
    // Chart::build(ds)? // Equivalent to chart!(ds)?
    //     .mark_point()?
    //     .encode((alt::x("height"), alt::y("weight")))?
    //     .save("out.svg")?;
    // load_polars_df!()
    // let ds = Dataset::new(vals);

    // Chart::build(vals);

    // println!("{:#?}", vals);
    println!("Done.");
    Ok(())
}
