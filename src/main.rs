use clap::Parser;

mod cli;
mod common;
mod solvers;

use cli::{Cli, SolverMode};
use common::parser;
use solvers::exact;
use solvers::heuristic;
use std::time::Instant;

fn main() {
    let args = Cli::parse();

    println!("Mode: {:?}, Input: {:?}", args.mode, args.input);

    let instance = match parser::load_instance(&args.input) {
        Ok(inst) => inst,
        Err(e) => {
            eprintln!("Fail to load instance {:?}: {}", args.input, e);
            std::process::exit(1);
        }
    };

    println!(
        "Instance loaded with success (nodes {}, subgroups {}, clusters, {} vehicles {})",
        instance.nodes.len(),
        instance.subgroups.len(),
        instance.clusters.len(),
        instance.vehicles.len()
    );

    let start_time = Instant::now();

    let solution = match args.mode {
        SolverMode::Exact => exact::solve(instance),
        SolverMode::Alns => heuristic::solve(instance),
    };

    let solution = match solution {
        Ok(sol) => sol,
        Err(e) => {
            eprintln!("Fail to solve the instance: {}", e);
            std::process::exit(1);
        }
    };

    let duration = start_time.elapsed();

    println!("--- END OF PROCESSING ---");
    println!("Execution Time: {:.2?}", duration);
    println!("Status: {:?}", solution.status);
    println!("Objective (Total Score): {}", solution.total_score);
}
