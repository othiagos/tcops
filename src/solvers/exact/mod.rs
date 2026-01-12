use crate::common::{error::SolverError, instance::Instance, solution::Solution};

pub mod ilp;

use ilp::Ilp;

pub fn solve(instance: Instance) -> Result<Solution, SolverError> {
    println!("(EXACT) Running solver");

    let solver = Ilp::new(instance);
    solver.solve()
}
