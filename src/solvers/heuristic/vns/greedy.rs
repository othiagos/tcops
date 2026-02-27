use crate::common::{
    constants::EPSILON,
    error::SolverError,
    instance::Instance,
    solution::{Route, Solution, SolutionStatus},
};
use crate::solvers::heuristic::vns::{
    neighborhoods::evaluate_subgroup_insertion, state::SearchState,
};

pub fn build_greedy_solution(instance: &'_ Instance) -> Result<(Solution<'_>, SearchState), SolverError> {
    let (mut solution, mut state) = initialize_empty_solution(instance);

    greedily_insert_subgroups(instance, &mut solution, &mut state);

    Ok((solution, state))
}

fn initialize_empty_solution(instance: &'_ Instance) -> (Solution<'_>, SearchState) {
    let mut state = SearchState::default();
    let mut routes = Vec::with_capacity(instance.vehicles.len());
    let mut initial_total_cost = 0.0;

    for vehicle in &instance.vehicles {
        let start = vehicle.start_node_id;
        let end = vehicle.end_node_id;

        state.visited_nodes.insert(start);
        state.visited_nodes.insert(end);

        for &sg_id in &instance.nodes[start].parent_subgroup_ids {
            let c_id = instance.subgroups[sg_id].parent_cluster_id;
            state.cluster_locks.insert(c_id, sg_id);
            state.subgroup_nodes_count.insert(sg_id, 1);
        }

        let base_cost = instance.get_distance(start, end);
        initial_total_cost += base_cost;

        routes.push(Route {
            path: vec![start, end],
            cost: base_cost,
            score: 0.0,
            vehicle_id: vehicle.id,
        });
    }

    let solution = Solution {
        instance,
        total_score: 0.0,
        total_cost: initial_total_cost,
        routes,
        status: SolutionStatus::Feasible,
    };

    (solution, state)
}

fn greedily_insert_subgroups(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
) {
    while let Some((new_sol, new_state)) = find_best_subgroup_insertion(instance, solution, state) {
        *solution = new_sol;
        *state = new_state;
    }
}

fn find_best_subgroup_insertion<'a>(
    instance: &Instance,
    solution: &Solution<'a>,
    state: &SearchState,
) -> Option<(Solution<'a>, SearchState)> {
    let mut best_trial = None;
    let mut best_ratio = -1.0;

    for subgroup_id in 0..instance.subgroups.len() {
        if state.subgroup_nodes_count.contains_key(&subgroup_id) {
            continue;
        }

        if let Some((trial_sol, trial_state)) =
            evaluate_subgroup_insertion(instance, solution, state, subgroup_id)
        {
            let delta_cost = trial_sol.total_cost - solution.total_cost;
            let profit = instance.subgroups[subgroup_id].profit;
            let ratio = profit / (delta_cost + EPSILON);

            if ratio > best_ratio {
                best_ratio = ratio;
                best_trial = Some((trial_sol, trial_state));
            }
        }
    }

    best_trial
}
