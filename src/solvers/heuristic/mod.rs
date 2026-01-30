use crate::common::{error::SolverError, instance::Instance, solution::Solution};

#[derive(Default)]
pub struct _AlnsConfig {
    pub max_time_seconds: u64,
}

pub fn solve(_instance: Instance) -> Result<Solution, SolverError> {
    Ok(Solution::default())
}
