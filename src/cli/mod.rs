use clap::{Parser, ValueEnum, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "TCOPS Solver Engine", long_about = "Resolution engine for the Team Clustered Orienteering Problem with Subgroups.")]
pub struct Cli {
    /// Path to the instance file (.tcops)
    #[arg(long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub input: PathBuf,

    /// Algorithm execution mode (exact or heuristic)
    #[arg(long, value_enum)]
    pub mode: SolverMode,

    /// Mathematical solver type (Required if mode=exact)
    #[arg(long, value_enum, required_if_eq("mode", "exact"))]
    pub solver: Option<ExactSolverType>,

    /// Maximum iterations without improvement for the VNS (Only for mode=heuristic)
    #[arg(long, default_value_t = 100)]
    pub max_iterations: usize,

    /// Displays the detailed solution in the terminal at the end of the execution
    #[arg(long, default_value_t = false)]
    pub show: bool,

    /// Saves the solution result to an output file
    #[arg(long, default_value_t = false)]
    pub save: bool,

}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SolverMode {
    Exact,
    Heuristic,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ExactSolverType {
    Gurobi,
    Highs,
}