use crate::{
    cli::ExactSolverType,
    common::{error::SolverError, instance::Instance, solution::Solution},
};

use good_lp::{
    LpSolver,
    solvers::{highs::highs, lp_solvers::GurobiSolver},
};

mod constraint;
mod ilp;
mod objective;
mod parser;
mod variable;

use ilp::Ilp;

pub fn solve(instance: Instance, solver_type: ExactSolverType) -> Result<Solution, SolverError> {
    let ilp = Ilp::new(instance);

    match solver_type {
        ExactSolverType::Gurobi => ilp.solve(LpSolver(GurobiSolver::new())),
        ExactSolverType::Highs => ilp.solve(highs),
    }
}
