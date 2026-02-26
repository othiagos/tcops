use crate::common::{instance::Instance, solution::Solution};
use crate::solvers::heuristic::vns::state::SearchState;

#[derive(Debug, Clone)]
pub struct InsertMove {
    pub vehicle_id: usize,
    pub insert_index: usize,
    pub node_id: usize,
    pub subgroup_id: usize,
    pub cluster_id: usize,
    pub delta_cost: f64,
    pub delta_score: f64,
}

#[derive(Debug, Clone)]
pub struct DropMove {
    pub vehicle_id: usize,
    pub drop_index: usize,
    pub node_id: usize,
    pub subgroup_id: usize,
    pub cluster_id: usize,
    pub delta_cost: f64,
    pub delta_score: f64,
}

pub fn evaluate_insertion(
    instance: &Instance,
    solution: &Solution,
    state: &SearchState,
    vehicle_id: usize,
    node_id: usize,
) -> Option<InsertMove> {
    let mut valid_sg_id = None;
    let mut valid_cluster_id = None;

    for &sg_id in &instance.nodes[node_id].parent_subgroup_ids {
        let cluster_id = instance.subgroups[sg_id].parent_cluster_id;

        if let Some(&locked_sg) = state.cluster_locks.get(&cluster_id) {
            if locked_sg == sg_id {
                valid_sg_id = Some(sg_id);
                valid_cluster_id = Some(cluster_id);
                break;
            }
        } else {
            valid_sg_id = Some(sg_id);
            valid_cluster_id = Some(cluster_id);
            break;
        }
    }

    let subgroup_id = valid_sg_id?;
    let cluster_id = valid_cluster_id?;

    let route = &solution.routes[vehicle_id];
    let mut best_move = None;
    let mut best_ratio = -1.0;

    let node_profit = instance.nodes[node_id].profit;

    for i in 0..(route.path.len() - 1) {
        let prev_node = route.path[i];
        let next_node = route.path[i + 1];

        let added_dist =
            instance.get_distance(prev_node, node_id) + instance.get_distance(node_id, next_node);
        let removed_dist = instance.get_distance(prev_node, next_node);
        let delta_cost = added_dist - removed_dist;

        let vehicle = &instance.vehicles[route.vehicle_id];
        if route.cost + delta_cost <= vehicle.budget {
            let ratio = node_profit / (delta_cost + 1e-6);

            if ratio > best_ratio {
                best_ratio = ratio;
                best_move = Some(InsertMove {
                    vehicle_id: route.vehicle_id,
                    insert_index: i + 1,
                    node_id,
                    subgroup_id,
                    cluster_id,
                    delta_cost,
                    delta_score: node_profit,
                });
            }
        }
    }

    best_move
}

pub fn apply_insertion(solution: &mut Solution, state: &mut SearchState, insert_move: &InsertMove) {
    let route = &mut solution.routes[insert_move.vehicle_id];

    route
        .path
        .insert(insert_move.insert_index, insert_move.node_id);
    route.cost += insert_move.delta_cost;
    route.score += insert_move.delta_score;

    solution.total_cost += insert_move.delta_cost;
    solution.total_score += insert_move.delta_score;

    state.visited_nodes.insert(insert_move.node_id);
    state
        .cluster_locks
        .insert(insert_move.cluster_id, insert_move.subgroup_id);

    *state
        .subgroup_nodes_count
        .entry(insert_move.subgroup_id)
        .or_insert(0) += 1;
}

pub fn evaluate_drop(
    instance: &Instance,
    solution: &Solution,
    state: &SearchState,
    vehicle_id: usize,
    drop_index: usize,
) -> Option<DropMove> {
    let route = &solution.routes[vehicle_id];

    if drop_index == 0 || drop_index == route.path.len() - 1 {
        return None;
    }

    let node_id = route.path[drop_index];
    let prev_node = route.path[drop_index - 1];
    let next_node = route.path[drop_index + 1];

    let mut active_sg = None;
    let mut active_cluster = None;

    for &sg_id in &instance.nodes[node_id].parent_subgroup_ids {
        let cluster_id = instance.subgroups[sg_id].parent_cluster_id;
        if state.cluster_locks.get(&cluster_id) == Some(&sg_id) {
            active_sg = Some(sg_id);
            active_cluster = Some(cluster_id);
            break;
        }
    }

    let subgroup_id = active_sg?;
    let cluster_id = active_cluster?;

    let removed_dist =
        instance.get_distance(prev_node, node_id) + instance.get_distance(node_id, next_node);
    let added_dist = instance.get_distance(prev_node, next_node);

    let delta_cost = added_dist - removed_dist;
    let delta_score = -instance.nodes[node_id].profit;

    Some(DropMove {
        vehicle_id,
        drop_index,
        node_id,
        subgroup_id,
        cluster_id,
        delta_cost,
        delta_score,
    })
}

pub fn apply_drop(solution: &mut Solution, state: &mut SearchState, drop_move: &DropMove) {
    let route = &mut solution.routes[drop_move.vehicle_id];

    route.path.remove(drop_move.drop_index);
    route.cost += drop_move.delta_cost;
    route.score += drop_move.delta_score;

    solution.total_cost += drop_move.delta_cost;
    solution.total_score += drop_move.delta_score;
    state.visited_nodes.remove(&drop_move.node_id);

    if let Some(count) = state.subgroup_nodes_count.get_mut(&drop_move.subgroup_id) {
        *count -= 1;
        if *count == 0 {
            state.subgroup_nodes_count.remove(&drop_move.subgroup_id);
            state.cluster_locks.remove(&drop_move.cluster_id);
        }
    }
}
