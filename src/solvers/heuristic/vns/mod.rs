use crate::common::{error::SolverError, instance::Instance, solution::Solution};

mod greedy;
mod local_search;
mod neighborhoods;
mod shaking;
mod solver;
mod state;

pub fn solve(
    instance: Instance,
    max_iterations: usize,
    max_shaking_intensity: usize,
) -> Result<Solution, SolverError> {
    solver::solve(instance, max_iterations, max_shaking_intensity)
}
