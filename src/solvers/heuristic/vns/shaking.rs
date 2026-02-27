use rand::Rng;
use rand::seq::SliceRandom;

use crate::{
    common::{instance::Instance, solution::Solution},
    solvers::heuristic::vns::{
        neighborhoods::{drop_subgroup, evaluate_subgroup_insertion},
        state::SearchState,
    },
};

pub fn apply_shaking<R: Rng>(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
    rng: &mut R,
    shaking_intensity: usize,
) {
    apply_destruction_phase(instance, solution, state, rng, shaking_intensity);
    apply_kick_phase(instance, solution, state, rng, shaking_intensity);
}

fn apply_destruction_phase<R: Rng>(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
    rng: &mut R,
    shaking_intensity: usize,
) {
    let active_subgroups: Vec<usize> = state.subgroup_nodes_count.keys().copied().collect();

    if active_subgroups.is_empty() {
        return;
    }

    let amount_to_drop = shaking_intensity.min(active_subgroups.len());
    let to_remove: Vec<&usize> = active_subgroups
        .choose_multiple(rng, amount_to_drop)
        .collect();

    for &sg_id in to_remove {
        drop_subgroup(instance, solution, state, sg_id);
    }
}

fn apply_kick_phase<R: Rng>(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
    rng: &mut R,
    shaking_intensity: usize,
) {
    const MIN_SHAKING_INTENSITY: usize = 2;
    if shaking_intensity < MIN_SHAKING_INTENSITY {
        return;
    }

    let unvisited_subgroups: Vec<usize> = (0..instance.subgroups.len())
        .filter(|subgroup_id| !state.subgroup_nodes_count.contains_key(subgroup_id))
        .collect();

    if let Some(&random_subgroup) = unvisited_subgroups.choose(rng)
        && let Some((new_sol, new_state)) =
            evaluate_subgroup_insertion(instance, solution, state, random_subgroup)
    {
        *solution = new_sol;
        *state = new_state;
    }
}
