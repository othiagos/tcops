use crate::common::{instance::Instance, solution::Solution};
use crate::solvers::heuristic::vns::{
    neighborhoods::evaluate_subgroup_insertion, state::SearchState,
};

pub fn local_search_insertions(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
) {
    while let Some((new_sol, new_state)) = find_best_improving_insertion(instance, solution, state)
    {
        *solution = new_sol;
        *state = new_state;
    }
}

fn find_best_improving_insertion(
    instance: &Instance,
    solution: &Solution,
    state: &SearchState,
) -> Option<(Solution, SearchState)> {
    let mut best_trial = None;
    let mut best_obj_value = solution.get_objective_value();

    for subgroup_id in 0..instance.subgroups.len() {
        if state.subgroup_nodes_count.contains_key(&subgroup_id) {
            continue;
        }

        if let Some((trial_sol, trial_state)) =
            evaluate_subgroup_insertion(instance, solution, state, subgroup_id)
        {
            let trial_obj_value = trial_sol.get_objective_value();

            if trial_obj_value > best_obj_value {
                best_obj_value = trial_obj_value;
                best_trial = Some((trial_sol, trial_state));
            }
        }
    }

    best_trial
}
