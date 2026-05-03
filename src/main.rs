use clap::Parser;

mod cli;
mod common;
mod exporter;
mod parser;
mod plotter;
mod printer;
mod runner;
mod solvers;

use cli::Cli;

fn main() {
    let args = Cli::parse();

    if let Err(e) = runner::run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
