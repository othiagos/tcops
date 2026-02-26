use rand::Rng;
use rand::seq::SliceRandom;

use crate::{
    common::{instance::Instance, solution::Solution},
    solvers::heuristic::vns::{
        neighborhoods::{apply_drop, apply_insertion, evaluate_drop, evaluate_insertion},
        state::SearchState,
    },
};

pub fn apply_shaking<R: Rng>(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
    k: usize,
    rng: &mut R,
) {
    apply_destruction_phase(instance, solution, state, k, rng);
    apply_kick_phase(instance, solution, state, k, rng);
}

fn apply_destruction_phase<R: Rng>(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
    k: usize,
    rng: &mut R,
) {
    let mut drops_performed = 0;
    let max_attempts = k * 10;
    let mut attempts = 0;

    while drops_performed < k && attempts < max_attempts {
        attempts += 1;

        if solution.routes.is_empty() {
            break;
        }

        let v_idx = rng.gen_range(0..solution.routes.len());
        let route = &solution.routes[v_idx];

        if route.path.len() <= 2 {
            continue;
        }

        let drop_index = rng.gen_range(1..(route.path.len() - 1));

        if let Some(drop_move) = evaluate_drop(instance, solution, state, v_idx, drop_index) {
            apply_drop(solution, state, &drop_move);
            drops_performed += 1;
        }
    }
}

fn apply_kick_phase<R: Rng>(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
    k: usize,
    rng: &mut R,
) {
    if k < 2 {
        return;
    }

    let empty_vehicles: Vec<usize> = solution
        .routes
        .iter()
        .enumerate()
        .filter(|(_, r)| r.path.len() <= 2)
        .map(|(i, _)| i)
        .collect();

    if empty_vehicles.is_empty() {
        return;
    }
   
    let target_vehicle = empty_vehicles[rng.gen_range(0..empty_vehicles.len())];

    let mut unvisited_nodes: Vec<usize> = (1..instance.nodes.len())
        .filter(|n| !state.visited_nodes.contains(n))
        .collect();

    unvisited_nodes.shuffle(rng);

    for &node_id in &unvisited_nodes {
        if let Some(insert_move) =
            evaluate_insertion(instance, solution, state, target_vehicle, node_id)
        {
            apply_insertion(solution, state, &insert_move);
            break;
        }
    }
}