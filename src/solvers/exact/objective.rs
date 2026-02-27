use crate::common::constants::DISTANCE_PENALTY;
use crate::common::instance::Instance;
use crate::solvers::exact::ilp::DecisionVariables;

use good_lp::Expression;

pub fn function(variables: &DecisionVariables, instance: &Instance) -> Expression {
    let num_vehicles = instance.vehicles.len();
    let num_nodes = instance.nodes.len();
    let num_subgroups = instance.subgroups.len();

    let mut objective = Expression::from(0.0);
    for s in 0..num_subgroups {
        objective += variables.z[s] * instance.subgroups[s].profit;
    }

    let mut total_dist_expr = Expression::from(0.0);

    for k in 0..num_vehicles {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if i != j {
                    let dist = instance.get_distance(i, j);
                    total_dist_expr += variables.x[k][i][j] * dist;
                }
            }
        }
    }

    objective -= total_dist_expr * DISTANCE_PENALTY;

    objective
}
