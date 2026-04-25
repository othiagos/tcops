use grb::prelude::*;
use crate::common::instance::Instance;

use crate::solvers::exact::gurobi::ilp::DecisionVariables;

pub fn set_objective(
    model: &mut Model,
    variable: &DecisionVariables,
    instance: &Instance,
) -> grb::Result<()> {
    
    let obj_expr: grb::Expr = (0..instance.subgroups.len())
        .map(|s| instance.subgroups[s].profit * variable.z[s])
        .sum();

    model.set_objective(obj_expr, Maximize)?;

    Ok(())
}