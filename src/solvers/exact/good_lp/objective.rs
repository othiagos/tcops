use crate::common::instance::Instance;
use crate::solvers::exact::good_lp::ilp::DecisionVariables;

use good_lp::Expression;

pub fn function(variables: &DecisionVariables, instance: &Instance) -> Expression {
    let num_subgroups = instance.subgroups.len();

    let mut objective = Expression::from(0.0);
    for s in 0..num_subgroups {
        objective += variables.z[s] * instance.subgroups[s].profit;
    }

    objective
}
