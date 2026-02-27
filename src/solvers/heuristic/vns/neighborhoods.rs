use core::f64;

use crate::{
    common::{constants::EPSILON, instance::Instance, solution::Solution},
    solvers::heuristic::vns::state::SearchState,
};

struct InsertionSpot {
    vehicle_id: usize,
    path_id: usize,
    cost_delta: f64,
}

pub fn evaluate_subgroup_insertion(
    instance: &Instance,
    solution: &Solution,
    state: &SearchState,
    subgroup_id: usize,
) -> Option<(Solution, SearchState)> {
    let cluster_id = instance.subgroups[subgroup_id].parent_cluster_id;

    if let Some(&locked_sg) = state.cluster_locks.get(&cluster_id)
        && locked_sg != subgroup_id
    {
        return None;
    }

    let mut trial_sol = solution.clone();
    let mut trial_state = state.clone();

    for &node_id in &instance.subgroups[subgroup_id].node_ids {
        match find_best_spot_for_node(instance, &trial_sol, node_id) {
            Some(spot) => {
                trial_sol.routes[spot.vehicle_id]
                    .path
                    .insert(spot.path_id, node_id);

                trial_sol.routes[spot.vehicle_id].cost += spot.cost_delta;
                trial_sol.total_cost += spot.cost_delta;
                trial_state.visited_nodes.insert(node_id);
                trial_sol.routes[spot.vehicle_id].score += instance.nodes[node_id].profit;
            }
            None => return None,
        }
    }

    trial_sol.total_score += instance.subgroups[subgroup_id].profit;
    trial_state.cluster_locks.insert(cluster_id, subgroup_id);
    trial_state
        .subgroup_nodes_count
        .insert(subgroup_id, instance.subgroups[subgroup_id].node_ids.len());

    Some((trial_sol, trial_state))
}

fn find_best_spot_for_node(
    instance: &Instance,
    solution: &Solution,
    node_id: usize,
) -> Option<InsertionSpot> {
    
    if instance.vehicles.iter().any(|v| v.start_node_id == node_id || v.end_node_id == node_id) {
        return None;
    }

    let mut best_spot = None;
    let mut best_cost = f64::MAX;

    for (vehicle_id, route) in solution.routes.iter().enumerate() {
        let vehicle = &instance.vehicles[vehicle_id];

        for i in 0..(route.path.len() - 1) {
            let prev = route.path[i];
            let next = route.path[i + 1];
            
            let added = instance.get_distance(prev, node_id) + instance.get_distance(node_id, next);
            let removed = instance.get_distance(prev, next);
            let delta = added - removed;

            if route.cost + delta <= vehicle.budget && delta < best_cost - EPSILON {
                best_cost = delta;
                best_spot = Some(InsertionSpot {
                    vehicle_id,
                    path_id: i + 1,
                    cost_delta: delta,
                });
            }
        }
    }

    best_spot
}

pub fn drop_subgroup(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
    subgroup_id: usize,
) {
    let cluster_id = instance.subgroups[subgroup_id].parent_cluster_id;

    for &node_id in &instance.subgroups[subgroup_id].node_ids {
        remove_node_from_routes(instance, solution, state, node_id);
    }

    solution.total_score -= instance.subgroups[subgroup_id].profit;
    state.cluster_locks.remove(&cluster_id);
    state.subgroup_nodes_count.remove(&subgroup_id);
}

fn remove_node_from_routes(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
    node_id: usize,
) {
    if instance
        .vehicles
        .iter()
        .any(|v| v.start_node_id == node_id || v.end_node_id == node_id)
    {
        return;
    }

    for route in &mut solution.routes {
        let intermediate_nodes = &route.path[1..route.path.len() - 1];

        if let Some(internal_pos) = intermediate_nodes.iter().position(|&n| n == node_id) {
            let pos = internal_pos + 1;

            let prev = route.path[pos - 1];
            let next = route.path[pos + 1];

            let removed_distance =
                instance.get_distance(prev, node_id) + instance.get_distance(node_id, next);
            let direct_distance = instance.get_distance(prev, next);
            let delta = direct_distance - removed_distance;

            route.path.remove(pos);
            route.cost += delta;
            solution.total_cost += delta;
            state.visited_nodes.remove(&node_id);
            route.score -= instance.nodes[node_id].profit;

            break;
        }
    }
}
