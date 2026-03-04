use crate::{
    cli::{Cli, ExactSolverType},
    common::{error::SolverError, instance::Instance, solution::Solution},
};

use good_lp::{
    LpSolver,
    solvers::{
        lp_solvers::{GurobiSolver, Model},
        scip::{SCIPProblem, scip},
    },
};

mod constraint;
mod ilp;
mod objective;
mod parser;
mod variable;

use ilp::Ilp;

fn gurobi_configure(config: Model<GurobiSolver>) -> Model<GurobiSolver> {
    config
}

fn scip_configure(config: SCIPProblem) -> SCIPProblem {
    config
}

pub fn solve<'a>(instance: &'a Instance, args: &Cli) -> Result<Solution<'a>, SolverError> {
    let ilp = Ilp::new(instance);

    match args.solver.unwrap() {
        ExactSolverType::Gurobi => ilp.solve(LpSolver(GurobiSolver::new()), gurobi_configure),
        ExactSolverType::Scip => ilp.solve(scip, scip_configure),
    }
}
