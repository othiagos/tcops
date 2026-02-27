use crate::{
    cli::Cli,
    common::{error::SolverError, instance::Instance, solution::Solution},
};

mod vns;

pub fn solve<'a>(instance: &'a Instance, args: &Cli) -> Result<Solution<'a>, SolverError> {
    vns::solve(instance, args.max_iterations, args.max_shaking_intensity)
}
