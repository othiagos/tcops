use crate::{
    cli::Cli,
    common::{error::SolverError, instance::Instance, solution::Solution},
};

mod vns;

pub fn solve(instance: Instance, args: &Cli) -> Result<Solution, SolverError> {
    vns::solve(instance, args.max_iterations, args.max_shaking_intensity)
}
