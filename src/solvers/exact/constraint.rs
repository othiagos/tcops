use std::collections::HashSet;

use good_lp::{Constraint, Expression};

use crate::common::instance::Instance;

use crate::solvers::exact::ilp::DecisionVariables;

pub fn flow_conservation(variable: &DecisionVariables, instance: &Instance) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    let num_nodes = instance.nodes.len();

    for k in 0..instance.vehicles.len() {
        let start_node = instance.vehicles[k].start_node_id;
        let end_node = instance.vehicles[k].end_node_id;

        for i in 0..num_nodes {
            let mut sum_in = Expression::from(0.0);
            for j in 0..num_nodes {
                if i != j {
                    sum_in += variable.x[k][j][i];
                }
            }

            let mut sum_out = Expression::from(0.0);
            for j in 0..num_nodes {
                if i != j {
                    sum_out += variable.x[k][i][j];
                }
            }

            if start_node == end_node {
                if i == start_node {
                    constraints.push(sum_out.eq(1.0));
                    constraints.push(sum_in.eq(1.0));
                } else {
                    constraints.push(sum_in.eq(variable.y[k][i]));
                    constraints.push(sum_out.eq(variable.y[k][i]));
                }
            } else if i == start_node {
                constraints.push(sum_out.eq(1.0));
                constraints.push(sum_in.eq(0.0));
            } else if i == end_node {
                constraints.push(sum_in.eq(1.0));
                constraints.push(sum_out.eq(0.0));
            } else {
                constraints.push(sum_in.eq(variable.y[k][i]));
                constraints.push(sum_out.eq(variable.y[k][i]));
            }
        }
    }
    constraints
}

pub fn unique_visit(variable: &DecisionVariables, instance: &Instance) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    let num_nodes = instance.nodes.len();

    let mut depot_nodes = HashSet::new();
    for vehicle in instance.vehicles.iter() {
        depot_nodes.insert(vehicle.start_node_id);
        depot_nodes.insert(vehicle.end_node_id);
    }

    for i in 0..num_nodes {
        if depot_nodes.contains(&i) {
            continue;
        }

        let mut total_visits = Expression::from(0.0);

        for k in 0..instance.vehicles.len() {
            total_visits += variable.y[k][i];
        }

        constraints.push(total_visits.leq(1.0));
    }

    constraints
}

pub fn logical_physical(variable: &DecisionVariables, instance: &Instance) -> Vec<Constraint> {
    let mut constraints = Vec::new();

    for (i, node) in instance.nodes.iter().enumerate() {
        let mut sum_z_logic = Expression::from(0.0);
        for &s_id in node.parent_subgroup_ids.iter() {
            sum_z_logic += variable.z[s_id];
        }

        let mut sum_y_physic = Expression::from(0.0);
        for k in 0..instance.vehicles.len() {
            sum_y_physic += variable.y[k][i];
        }

        constraints.push(sum_z_logic.eq(sum_y_physic));
    }

    constraints
}

pub fn cluster(variable: &DecisionVariables, instance: &Instance) -> Vec<Constraint> {
    let mut constraints = Vec::new();

    for (c_id, cluster) in instance.clusters.iter().enumerate() {
        let mut sum_z_subgroups = Expression::from(0.0);

        for &subgroup_id in &cluster.subgroup_ids {
            sum_z_subgroups += variable.z[subgroup_id];
        }

        constraints.push(sum_z_subgroups.eq(variable.w[c_id]));
    }

    constraints
}

pub fn budget(variable: &DecisionVariables, instance: &Instance) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    let num_nodes = instance.nodes.len();

    for k in 0..instance.vehicles.len() {
        let mut total_cost_expr = Expression::from(0.0);
        let vehicle_budget = instance.vehicles[k].budget;

        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if i != j {
                    let dist = instance.get_distance(i, j);
                    total_cost_expr += variable.x[k][i][j] * dist;
                }
            }
        }

        constraints.push(total_cost_expr.leq(vehicle_budget));
    }

    constraints
}

pub fn subtour_elimination_mtz(
    variable: &DecisionVariables,
    instance: &Instance,
) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    let n = instance.nodes.len() as f64;

    let mut depot_nodes = HashSet::new();
    for vehicle in instance.vehicles.iter() {
        depot_nodes.insert(vehicle.start_node_id);
        depot_nodes.insert(vehicle.end_node_id);
    }

    for k in 0..instance.vehicles.len() {
        for i in 0..instance.nodes.len() {
            for j in 0..instance.nodes.len() {
                if depot_nodes.contains(&i) || depot_nodes.contains(&j) {
                    continue;
                }

                if i != j {
                    let expr = variable.u[k][i] - variable.u[k][j] + n * variable.x[k][i][j];
                    constraints.push(expr.leq(n - 1.0));
                }
            }
        }
    }

    constraints
}
