use std::mem;

use crate::common::error::{SolverError, SolverErrorKind};
use crate::common::instance::Vehicle;
use crate::common::{
    instance::Instance,
    solution::{Route, Solution, SolutionStatus},
};

use crate::solvers::heuristic::vns::state::SearchState;

struct VehicleState {
    current_node: usize,
    cost: f64,
    score: f64,
    path: Vec<usize>,
    active: bool,
}

struct BestNode {
    step_cost: f64,
    node_id: usize,
    subgroup_id: usize,
    cluster_id: usize,
}

pub fn build_greedy_solution(instance: &Instance) -> Result<(Solution, SearchState), SolverError> {
    if instance.vehicles.is_empty() {
        return Err(SolverError::new(
            SolverErrorKind::Solver,
            "No vehicles available in the instance",
        ));
    }

    let mut state = SearchState::default();
    let mut vehicle_states = Vec::new();

    initialize_vehicle_states(instance, &mut state, &mut vehicle_states);

    assign_nodes(instance, &mut state, &mut vehicle_states);

    let (routes, total_cost, total_score) = finalize_routes(instance, &mut vehicle_states);

    let solution = Solution {
        instance: instance.clone(),
        total_score,
        total_cost,
        routes,
        status: SolutionStatus::Feasible,
    };

    Ok((solution, state))
}

fn initialize_vehicle_states(
    instance: &Instance,
    state: &mut SearchState,
    vehicle_states: &mut Vec<VehicleState>,
) {
    for vehicle in &instance.vehicles {
        let start_node = vehicle.start_node_id;
        state.visited_nodes.insert(start_node);

        for &start_subgroup in &instance.nodes[start_node].parent_subgroup_ids {
            let start_cluster = instance.subgroups[start_subgroup].parent_cluster_id;

            state.cluster_locks.insert(start_cluster, start_subgroup);
            *state
                .subgroup_nodes_count
                .entry(start_subgroup)
                .or_insert(0) += 1;
        }

        vehicle_states.push(VehicleState {
            current_node: start_node,
            cost: 0.0,
            score: 0.0,
            path: vec![start_node],
            active: true,
        });
    }
}

fn assign_nodes(instance: &Instance, state: &mut SearchState, vehicle_states: &mut [VehicleState]) {
    loop {
        let mut progress_made_in_this_round = false;

        for (vehicle_id, vehicle_state) in vehicle_states.iter_mut().enumerate() {
            if !vehicle_state.active {
                continue;
            }

            let vehicle = &instance.vehicles[vehicle_id];
            let best_find_node =
                find_best_node_for_vehicle(instance, state, vehicle_state, vehicle);

            match best_find_node {
                Some(best_node) => {
                    state.visited_nodes.insert(best_node.node_id);
                    state
                        .cluster_locks
                        .insert(best_node.cluster_id, best_node.subgroup_id);

                    *state
                        .subgroup_nodes_count
                        .entry(best_node.subgroup_id)
                        .or_insert(0) += 1;

                    vehicle_state.path.push(best_node.node_id);
                    vehicle_state.cost += best_node.step_cost;
                    vehicle_state.score += instance.nodes[best_node.node_id].profit;
                    vehicle_state.current_node = best_node.node_id;

                    progress_made_in_this_round = true;
                }
                None => vehicle_state.active = false,
            }
        }

        if !progress_made_in_this_round {
            break;
        }
    }
}

fn find_best_node_for_vehicle(
    instance: &Instance,
    state: &SearchState,
    vehicle_state: &VehicleState,
    vehicle: &Vehicle,
) -> Option<BestNode> {
    let end_node = vehicle.end_node_id;

    let mut best_ratio = -1.0;
    let mut best_step_cost = 0.0;
    let mut best_node = None;
    let mut best_node_subgroup = None;
    let mut best_node_cluster = None;

    for next_node in 0..instance.nodes.len() {
        if state.visited_nodes.contains(&next_node) || next_node == end_node {
            continue;
        }

        let step_cost = instance.get_distance(vehicle_state.current_node, next_node);
        let return_cost = instance.get_distance(next_node, end_node);

        if vehicle_state.cost + step_cost + return_cost <= vehicle.budget {
            let profit = instance.nodes[next_node].profit;
            let ratio = profit / (step_cost + 1e-6);

            for &subgroup_id in &instance.nodes[next_node].parent_subgroup_ids {
                let cluster_id = instance.subgroups[subgroup_id].parent_cluster_id;

                if let Some(&locked_subgroup) = state.cluster_locks.get(&cluster_id)
                    && locked_subgroup != subgroup_id
                {
                    continue;
                }

                if ratio > best_ratio {
                    best_ratio = ratio;
                    best_step_cost = step_cost;
                    best_node = Some(next_node);
                    best_node_subgroup = Some(subgroup_id);
                    best_node_cluster = Some(cluster_id);
                }
            }
        }
    }

    best_node.map(|node| BestNode {
        step_cost: best_step_cost,
        node_id: node,
        subgroup_id: best_node_subgroup.unwrap(),
        cluster_id: best_node_cluster.unwrap(),
    })
}

fn finalize_routes(
    instance: &Instance,
    vehicle_states: &mut [VehicleState],
) -> (Vec<Route>, f64, f64) {
    let mut routes = Vec::with_capacity(vehicle_states.len());
    let mut total_score = 0.0;
    let mut total_cost = 0.0;

    for (vehicle_id, vehicle) in instance.vehicles.iter().enumerate() {
        let vehicle_state = &mut vehicle_states[vehicle_id];
        let end_node = vehicle.end_node_id;

        vehicle_state.cost += instance.get_distance(vehicle_state.current_node, end_node);
        vehicle_state.path.push(end_node);

        total_cost += vehicle_state.cost;
        total_score += vehicle_state.score;

        routes.push(Route {
            path: mem::take(&mut vehicle_state.path),
            cost: vehicle_state.cost,
            score: vehicle_state.score,
            vehicle_id: vehicle.id,
        });
    }

    (routes, total_cost, total_score)
}
