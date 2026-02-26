use crate::common::instance::Instance;
use crate::common::solution::Solution;

use crate::solvers::heuristic::vns::{
    neighborhoods::{InsertMove, apply_insertion, evaluate_insertion},
    state::SearchState,
};

const EPSILON: f64 = 1e-4;

pub fn local_search_insertions(
    instance: &Instance,
    solution: &mut Solution,
    state: &mut SearchState,
) {
    let mut improvement = true;

    while improvement {
        improvement = false;
        let mut best_global_move: Option<InsertMove> = None;

        for next_node in 1..instance.nodes.len() {
            if state.visited_nodes.contains(&next_node) {
                continue;
            }

            for v_idx in 0..solution.routes.len() {
                if let Some(insert_move) =
                    evaluate_insertion(instance, solution, state, v_idx, next_node)
                {
                    let is_better = match &best_global_move {
                        None => true,
                        Some(best) => {
                            if insert_move.delta_score > best.delta_score + EPSILON {
                                true
                            } else if (insert_move.delta_score - best.delta_score).abs() < EPSILON {
                                insert_move.delta_cost < best.delta_cost - EPSILON
                            } else {
                                false
                            }
                        }
                    };

                    if is_better {
                        best_global_move = Some(insert_move);
                    }
                }
            }
        }

        if let Some(best_move) = best_global_move {
            apply_insertion(solution, state, &best_move);
            improvement = true;
        }
    }
}
