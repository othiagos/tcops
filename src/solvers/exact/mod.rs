use crate::{
    cli::{Cli, LibraryType},
    common::{error::SolverError, instance::Instance, solution::Solution},
};

pub mod good_lp;
pub mod gurobi;

pub fn solve<'a>(instance: &'a Instance, args: &Cli) -> Result<Solution<'a>, SolverError> {
    match args.library.unwrap() {
        LibraryType::Gurobi => gurobi::solve(instance, args),
        LibraryType::GoodLp => good_lp::solve(instance, args)
    }
}
