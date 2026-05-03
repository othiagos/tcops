use crate::cli::{Cli, SolverMode};
use crate::exporter::io;
use crate::parser;
use crate::plotter::plot;
use crate::printer;
use crate::solvers::{exact, heuristic};

pub fn run(args: Cli) -> Result<(), String> {
    println!("Mode: {:?}, Input: {:?}", args.mode, args.input);

    let instance = parser::load_instance(&args.input)
        .map_err(|e| format!("Fail to load instance {:?}\n{}", args.input, e))?;

    printer::print_instance_info(&instance);

    let solution = match args.mode {
        SolverMode::Exact => exact::solve(&instance, &args),
        SolverMode::Heuristic => heuristic::solve(&instance, &args),
    }
    .map_err(|e| format!("Fail to solve the instance: {}", e))?;

    printer::print_solution(&solution);

    let path = io::export_solution_to_json(&solution);

    if let Some(path) = path {
        plot::show(&path, args.show, args.save);
    }

    Ok(())
}
