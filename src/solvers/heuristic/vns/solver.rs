use rand::thread_rng;

use crate::{
    common::{error::SolverError, instance::Instance, solution::Solution},
    solvers::heuristic::vns::{
        greedy::build_greedy_solution, local_search::local_search_insertions,
        shaking::apply_shaking,
    },
};

pub fn solve(
    instance: Instance,
    max_iterations_without_improvement: usize,
) -> Result<Solution, SolverError> {
    let (mut best_solution, mut best_state) = build_greedy_solution(&instance)?;
    println!(
        "Initial solution found!  Objective: {:.2} | Score: {} | Cost: {:.2}",
        best_solution.get_objective_value(),
        best_solution.total_score,
        best_solution.total_cost
    );

    local_search_insertions(&instance, &mut best_solution, &mut best_state);

    let mut current_solution = best_solution.clone();
    let mut current_state = best_state.clone();

    let mut iter_without_improvement = 0;
    let mut rng = thread_rng();

    while iter_without_improvement < max_iterations_without_improvement {
        let mut k = 1;
        let k_max = 5;

        while k <= k_max {
            let mut trial_solution = current_solution.clone();
            let mut trial_state = current_state.clone();

            apply_shaking(
                &instance,
                &mut trial_solution,
                &mut trial_state,
                k,
                &mut rng,
            );

            local_search_insertions(&instance, &mut trial_solution, &mut trial_state);

            let trial_obj = trial_solution.get_objective_value();
            let current_obj = current_solution.get_objective_value();

            if trial_obj > current_obj {
                current_solution = trial_solution;
                current_state = trial_state;
                k = 1;
            } else {
                k += 1;
            }
        }

        let current_obj = current_solution.get_objective_value();
        let best_obj = best_solution.get_objective_value();

        if current_obj > best_obj {
            best_solution = current_solution.clone();
            iter_without_improvement = 0;
            println!(
                "New best solution found! Objective: {:.2} | Score: {} | Cost: {:.2}",
                current_obj, best_solution.total_score, best_solution.total_cost
            );
        } else {
            iter_without_improvement += 1;
        }
    }

    for route in &mut best_solution.routes {
        if route.path.len() == 2 && route.path[0] == route.path[1] {
            route.path.pop();
            route.cost = 0.0;
            route.score = 0.0;
        }
    }

    Ok(best_solution)
}
