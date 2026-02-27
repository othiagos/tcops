use crate::{
    cli::{Cli, ExactSolverType},
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

pub fn solve(instance: Instance, args: &Cli) -> Result<Solution, SolverError> {
    let ilp = Ilp::new(instance);

    match args.solver.unwrap() {
        ExactSolverType::Gurobi => ilp.solve(LpSolver(GurobiSolver::new())),
        ExactSolverType::Highs => ilp.solve(highs),
    }
}
