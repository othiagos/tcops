use crate::common::{error::SolverError, instance::Instance, solution::Solution};

mod constraint;
mod ilp;
mod objective;
mod parser;
mod variable;

use ilp::Ilp;

pub fn solve(instance: Instance) -> Result<Solution, SolverError> {
    println!("(EXACT) Running solver");

    Ilp::new(instance).solve()
}
