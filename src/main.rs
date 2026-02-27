use clap::Parser;

mod cli;
mod common;
mod exporter;
mod plotter;
mod solvers;

use cli::{Cli, SolverMode};
use common::parser;
use exporter::io;
use plotter::plot;
use solvers::exact;
use solvers::heuristic;
use std::time::Instant;

fn main() {
    let args = Cli::parse();

    println!("Mode: {:?}, Input: {:?}", args.mode, args.input);

    let (instance, folder_path) = match parser::load_instance(&args.input) {
        Ok((inst, folder)) => (inst, folder),
        Err(e) => {
            eprintln!("Fail to load instance {:?}: {}", args.input, e);
            std::process::exit(1);
        }
    };

    println!(
        "Instance loaded with success (nodes {}, subgroups {}, clusters {}, vehicles {})",
        instance.nodes.len(),
        instance.subgroups.len(),
        instance.clusters.len(),
        instance.vehicles.len()
    );

    let start_time = Instant::now();

    let solution = match args.mode {
        SolverMode::Exact => exact::solve(instance, &args),
        SolverMode::Heuristic => heuristic::solve(instance, &args)
    };

    let solution = match solution {
        Ok(sol) => sol,
        Err(e) => {
            eprintln!("Fail to solve the instance: {}", e);
            std::process::exit(1);
        }
    };

    let duration = start_time.elapsed();

    println!("Instance: {}", solution.instance.name);
    println!("Execution Time: {:.2?}", duration);
    println!("Status: {:?}", solution.status);
    println!("Objective Value: {:.2}", solution.get_objective_value());
    println!("Total Score: {:.2}", solution.total_score);
    println!("Total Cost: {:.2}", solution.total_cost);

    println!("Routes:");
    for route in &solution.routes {
        println!(
            "Vehicle {:02}: Cost: {:>8.2}, Score: {:>4.2}, Path: {:?}",
            route.vehicle_id, route.cost, route.score, route.path
        );
    }

    let filename = &solution.instance.name;
    let path = &format!("{}/{}.json", folder_path, filename);

    io::export_solution_to_json(path, &solution);
    plot::show(path, args.show, args.save);
}
