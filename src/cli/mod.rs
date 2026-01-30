use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "TCOPS Solver Engine", long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    #[arg(short, long, value_enum)]
    pub mode: SolverMode,

    #[arg(long, value_enum, required_if_eq("mode", "exact"))]
    pub solver: Option<ExactSolverType>,

    #[arg(short, long, default_value_t = false)]
    pub show: bool,

    #[arg(short, long, default_value_t = false)]
    pub save: bool,

    #[arg(short, long, default_value_t = 60)]
    pub limit: u64,
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
