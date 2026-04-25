use clap::{Parser, ValueEnum, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "TCOPS Solver Engine",
    long_about = "Resolution engine for the Team Clustered Orienteering Problem with Subgroups."
)]
pub struct Cli {
    /// Path to the instance file (.tcops)
    #[arg(long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub input: PathBuf,

    /// Algorithm execution mode (exact or heuristic)
    #[arg(long, value_enum)]
    pub mode: SolverMode,

    /// Mathematical library to use for the exact solver (Required if mode=exact)
    #[arg(long, value_enum, required_if_eq("mode", "exact"))]
    pub library: Option<LibraryType>,

    /// Mathematical solver type (Required if library=good-lp)
    #[arg(long, value_enum, required_if_eq("library", "good-lp"))]
    pub solver: Option<ExactSolverType>,

    /// Maximum iterations without improvement for the VNS (Only for mode=heuristic)
    #[arg(long, default_value_t = 100)]
    pub max_iterations: usize,

    /// Maximum number of subgroups to remove during the shaking phase
    #[arg(long, default_value_t = 20)]
    pub max_shaking_intensity: usize,

    /// Displays the detailed solution in the terminal at the end of the execution
    #[arg(long, default_value_t = false)]
    pub show: bool,

    /// Saves the solution result to an output file
    #[arg(long, default_value_t = false)]
    pub save: bool,

    /// Time limit for the exact solver gurobi in seconds (Only for mode=exact)
    #[arg(long)]
    pub time_limit: Option<u64>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SolverMode {
    Exact,
    Heuristic,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LibraryType {
    GoodLp,
    Gurobi,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ExactSolverType {
    Gurobi,
    Scip,
}
