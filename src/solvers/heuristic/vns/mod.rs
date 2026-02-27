use crate::common::{error::SolverError, instance::Instance, solution::Solution};

mod greedy;
mod local_search;
mod neighborhoods;
mod shaking;
mod solver;
mod state;

pub fn solve<'a>(
    instance: &'a Instance,
    max_iterations: usize,
    max_shaking_intensity: usize,
) -> Result<Solution<'a>, SolverError> {
    solver::solve(instance, max_iterations, max_shaking_intensity)
}
