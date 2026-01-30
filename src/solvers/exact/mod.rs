use crate::{
    cli::ExactSolverType,
    common::{error::SolverError, instance::Instance, solution::Solution},
};

mod constraint;
mod ilp;
mod objective;
mod parser;
mod variable;

use ilp::Ilp;

pub fn solve(instance: Instance, solver: ExactSolverType) -> Result<Solution, SolverError> {
    let ilp = Ilp::new(instance);

    match solver {
        ExactSolverType::Gurobi => ilp.solve_gurobi(),
        ExactSolverType::Highs => ilp.solve_highs(),
    }
}
