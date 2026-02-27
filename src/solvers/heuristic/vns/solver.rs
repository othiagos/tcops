use rand::thread_rng;

use crate::{
    common::{constants::EPSILON, error::SolverError, instance::Instance, solution::Solution},
    solvers::heuristic::vns::{
        greedy::build_greedy_solution, local_search::local_search_insertions,
        shaking::apply_shaking,
    },
};

pub fn solve<'a>(
    instance: &'a Instance,
    max_iterations_without_improvement: usize,
    max_shaking_intensity: usize,
) -> Result<Solution<'a>, SolverError> {
    let (mut best_solution, mut best_state) = build_greedy_solution(instance)?;
    local_search_insertions(instance, &mut best_solution, &mut best_state);

    println!(
        "Starting solution found! Objective: {:.2} | Score: {:.2} | Cost: {:.2}",
        best_solution.get_objective_value(),
        best_solution.total_score,
        best_solution.total_cost
    );

    let mut iter_without_improvement = 0;
    let mut rng = thread_rng();

    while iter_without_improvement < max_iterations_without_improvement {
        let mut shaking_intensity = 1;
        let mut improved_in_this_iteration = false;

        while shaking_intensity <= max_shaking_intensity {
            let mut trial_solution = best_solution.clone();
            let mut trial_state = best_state.clone();

            apply_shaking(
                instance,
                &mut trial_solution,
                &mut trial_state,
                &mut rng,
                shaking_intensity,
            );
            local_search_insertions(instance, &mut trial_solution, &mut trial_state);

            if trial_solution.get_objective_value() > best_solution.get_objective_value() + EPSILON
            {
                best_solution = trial_solution;
                best_state = trial_state;
                shaking_intensity = 1;
                improved_in_this_iteration = true;

                println!(
                    "New best solution found! Objective: {:.2} | Score: {:.2} | Cost: {:.2}",
                    best_solution.get_objective_value(),
                    best_solution.total_score,
                    best_solution.total_cost
                );
            } else {
                shaking_intensity += 1;
            }
        }

        if improved_in_this_iteration {
            iter_without_improvement = 0;
        } else {
            iter_without_improvement += 1;
        }
    }

    best_solution.total_cost = 0.0;

    for route in &mut best_solution.routes {
        if route.path.len() == 2 {
            route.path.truncate(1);
            route.cost = 0.0;
            route.score = 0.0;
        }

        best_solution.total_cost += route.cost;
    }

    Ok(best_solution)
}
