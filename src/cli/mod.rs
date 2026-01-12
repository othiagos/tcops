use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "TCOPS Solver Engine", long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    #[arg(short, long, value_enum)]
    pub mode: SolverMode,

    #[arg(short, long, default_value_t = 60)]
    pub time_limit: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SolverMode {
    Exact,
    Alns,
}