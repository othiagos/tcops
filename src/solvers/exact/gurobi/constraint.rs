#![allow(clippy::useless_conversion)]

use std::collections::HashSet;
use grb::prelude::*;

use crate::common::instance::Instance;
use crate::solvers::exact::gurobi::ilp::DecisionVariables; 

pub fn flow_conservation(
    model: &mut Model,
    variable: &DecisionVariables,
    instance: &Instance,
) -> grb::Result<()> {
    let num_nodes = instance.nodes.len();

    for k in 0..instance.vehicles.len() {
        let start_node = instance.vehicles[k].start_node_id;
        let end_node = instance.vehicles[k].end_node_id;

        for i in 0..num_nodes {
            let sum_in: grb::Expr = (0..num_nodes)
                .filter(|&j| i != j)
                .map(|j| 1.0 * variable.x[k][j][i])
                .sum();

            let sum_out: grb::Expr = (0..num_nodes)
                .filter(|&j| i != j)
                .map(|j| 1.0 * variable.x[k][i][j])
                .sum();

            let y_var = variable.y[k][i];

            if start_node == end_node {
                if i == start_node {
                    let out_clone = sum_out.clone();
                    model.add_constr(&format!("flow_out_limit_v{}_n{}", k, i), c!(out_clone <= 1.0))?;
                    model.add_constr(&format!("flow_cons_start_v{}_n{}", k, i), c!(sum_in == sum_out))?;
                } else {
                    model.add_constr(&format!("flow_in_y_v{}_n{}", k, i), c!(sum_in.clone() == y_var))?;
                    model.add_constr(&format!("flow_out_y_v{}_n{}", k, i), c!(sum_out == y_var))?;
                }
            } else if i == start_node {
                model.add_constr(&format!("flow_out_limit_start_v{}_n{}", k, i), c!(sum_out <= 1.0))?;
                model.add_constr(&format!("flow_in_zero_start_v{}_n{}", k, i), c!(sum_in == 0.0))?;
            } else if i == end_node {
                model.add_constr(&format!("flow_in_limit_end_v{}_n{}", k, i), c!(sum_in <= 1.0))?;
                model.add_constr(&format!("flow_out_zero_end_v{}_n{}", k, i), c!(sum_out == 0.0))?;
            } else {
                model.add_constr(&format!("flow_in_y_v{}_n{}", k, i), c!(sum_in.clone() == y_var))?;
                model.add_constr(&format!("flow_out_y_v{}_n{}", k, i), c!(sum_out == y_var))?;
            }

        }
    }
    Ok(())
}

pub fn unique_visit(
    model: &mut Model,
    variable: &DecisionVariables,
    instance: &Instance,
) -> grb::Result<()> {
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

        let total_visits: grb::Expr = (0..instance.vehicles.len())
            .map(|k| 1.0 * variable.y[k][i])
            .sum();

        model.add_constr(&format!("unique_visit_n{}", i), c!(total_visits <= 1.0_f32))?;
    }

    Ok(())
}

pub fn logical_physical(
    model: &mut Model,
    variable: &DecisionVariables,
    instance: &Instance,
) -> grb::Result<()> {
    for (i, node) in instance.nodes.iter().enumerate() {
        
        let sum_z_logic: grb::Expr = node.parent_subgroup_ids.iter()
            .map(|&s_id| 1.0 * variable.z[s_id])
            .sum();

        let sum_y_physic: grb::Expr = (0..instance.vehicles.len())
            .map(|k| 1.0 * variable.y[k][i])
            .sum();

        model.add_constr(&format!("logic_physic_n{}", i), c!(sum_z_logic == sum_y_physic))?;
    }

    Ok(())
}

pub fn cluster(
    model: &mut Model,
    variable: &DecisionVariables,
    instance: &Instance,
) -> grb::Result<()> {
    for (c_id, cluster) in instance.clusters.iter().enumerate() {
        
        let sum_z_subgroups: grb::Expr = cluster.subgroup_ids.iter()
            .map(|&subgroup_id| 1.0 * variable.z[subgroup_id])
            .sum();

        model.add_constr(&format!("cluster_c{}", c_id), c!(sum_z_subgroups == variable.w[c_id]))?;
    }

    Ok(())
}

pub fn budget(
    model: &mut Model,
    variable: &DecisionVariables,
    instance: &Instance,
) -> grb::Result<()> {
    let num_nodes = instance.nodes.len();

    for k in 0..instance.vehicles.len() {
        let vehicle_budget = instance.vehicles[k].budget;

        let total_cost_expr: grb::Expr = (0..num_nodes)
            .flat_map(|i| {
                (0..num_nodes)
                    .filter(move |&j| i != j)
                    .map(move |j| instance.get_distance(i, j) * variable.x[k][i][j])
            })
            .sum();

        model.add_constr(&format!("budget_v{}", k), c!(total_cost_expr <= vehicle_budget))?;
    }

    Ok(())
}

pub fn subtour_elimination_mtz(
    model: &mut Model,
    variable: &DecisionVariables,
    instance: &Instance,
) -> grb::Result<()> {
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
                    model.add_constr(
                        &format!("mtz_v{}_n{}_n{}", k, i, j),
                        c!(variable.u[k][i] - variable.u[k][j] + n * variable.x[k][i][j] <= n - 1.0)
                    )?;
                }
            }
        }
    }

    Ok(())
}