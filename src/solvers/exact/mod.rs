use crate::{
    cli::Cli,
    common::{error::SolverError, instance::Instance, solution::Solution},
};

pub mod good_lp;
pub mod gurobi;

pub fn solve<'a>(instance: &'a Instance, args: &Cli) -> Result<Solution<'a>, SolverError> {
    good_lp::solve(instance, args)
}
