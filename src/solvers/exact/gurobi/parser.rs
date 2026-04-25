use grb::prelude::*;

use crate::common::{
    instance::Instance,
    solution::{Route, Solution, SolutionStatus},
};

use crate::solvers::exact::gurobi::variable::DecisionVariables;

pub fn parse_solution<'a>(
    model: &Model,
    variables: &DecisionVariables,
    instance: &'a Instance,
) -> grb::Result<Solution<'a>> {
    let mut routes: Vec<Route> = Vec::new();

    for k in 0..instance.vehicles.len() {
        if let Some(route) = get_route(instance, model, variables, k)? {
            routes.push(route);
        }
    }

    let total_score: f64 = routes.iter().map(|r| r.score).sum();
    let total_cost: f64 = routes.iter().map(|r| r.cost).sum();

    Ok(Solution {
        instance,
        total_score,
        total_cost,
        routes,
        status: SolutionStatus::Optimal,
    })
}

fn get_route(
    instance: &Instance,
    model: &Model,
    variables: &DecisionVariables,
    k: usize,
) -> grb::Result<Option<Route>> {
    let current_route_nodes = get_route_node(instance, model, variables, k)?;

    if current_route_nodes.is_empty() {
        return Ok(None);
    }

    let mut route_cost = 0.0;
    let mut route_score = 0.0;

    for i in 0..current_route_nodes.len() - 1 {
        let current_id = current_route_nodes[i];

        route_score += instance.nodes[current_id].profit;

        let next_id = current_route_nodes[i + 1];
        route_cost += instance.get_distance(current_id, next_id);
    }

    let route = Route {
        path: current_route_nodes,
        cost: route_cost,
        score: route_score,
        vehicle_id: k,
    };

    Ok(Some(route))
}

fn get_route_node(
    instance: &Instance,
    model: &Model,
    variables: &DecisionVariables,
    k: usize,
) -> grb::Result<Vec<usize>> {
    let mut current_route_nodes: Vec<usize> = Vec::new();

    let mut current_node = instance.vehicles[k].start_node_id;
    let vehicle_end_node = instance.vehicles[k].end_node_id;
    current_route_nodes.push(current_node);

    let mut found_next;
    let num_nodes = instance.nodes.len();

    for _ in 0..num_nodes + 2 {
        found_next = false;

        for next_node in 0..num_nodes {
            if current_node == next_node {
                continue;
            }

            let var = &variables.x[k][current_node][next_node];
            let val = model.get_obj_attr(attr::X, var)?;

            if val >= 0.5 {
                current_route_nodes.push(next_node);
                current_node = next_node;
                found_next = true;
                break;
            }
        }

        if !found_next || current_node == vehicle_end_node {
            break;
        }
    }

    Ok(current_route_nodes)
}
