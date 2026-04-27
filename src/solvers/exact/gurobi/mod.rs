use crate::{
    cli::Cli,
    common::{
        error::{SolverError, SolverErrorKind},
        instance::Instance,
        solution::Solution,
    },
};

pub mod callback;
pub mod constraint;
pub mod ilp;
pub mod objective;
pub mod parser;
pub mod variable;

use ilp::Ilp;

pub fn solve<'a>(instance: &'a Instance, args: &Cli) -> Result<Solution<'a>, SolverError> {
    let ilp = Ilp::new(instance).map_err(|e| {
        SolverError::new(
            SolverErrorKind::Solver,
            &format!("Failed to build the Gurobi model: {}", e),
        )
    })?;

    ilp.solve(args)
}
