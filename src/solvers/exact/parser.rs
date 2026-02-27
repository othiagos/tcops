use good_lp::Solution as SolutionTrait;

use crate::common::{
    instance::Instance,
    solution::{Route, Solution, SolutionStatus},
};

use crate::solvers::exact::ilp::DecisionVariables;

pub fn parse_solution<S: SolutionTrait>(
    solution: S,
    variables: DecisionVariables,
    instance: Instance,
) -> Solution {
    let mut routes: Vec<Route> = Vec::new();
    for k in 0..instance.vehicles.len() {
        match get_route(&instance, &solution, &variables, k) {
            Some(route) => routes.push(route),
            None => continue,
        }
    }

    let total_score = routes.iter().map(|r| r.score).sum();
    let total_cost = routes.iter().map(|r| r.cost).sum();

    Solution {
        instance,
        total_score,
        total_cost,
        routes,
        status: SolutionStatus::Optimal,
    }
}

fn get_route<S: SolutionTrait>(
    instance: &Instance,
    solution: &S,
    variables: &DecisionVariables,
    k: usize,
) -> Option<Route> {
    let current_route_nodes = get_route_node(instance, solution, variables, k);

    if current_route_nodes.is_empty() {
        return None;
    }

    let mut route_cost = 0.0;
    let mut route_score = 0.0;

    for i in 0..current_route_nodes.len() - 1 {
        let current_id = current_route_nodes[i];

        route_score += instance.nodes[current_id].profit;

        let next_id = current_route_nodes[i + 1];
        route_cost += instance.get_distance(current_id, next_id);
    }

    let route = Route {
        path: current_route_nodes,
        cost: route_cost,
        score: route_score,
        vehicle_id: k,
    };

    Some(route)
}

fn get_route_node<S: SolutionTrait>(
    instance: &Instance,
    solution: &S,
    variables: &DecisionVariables,
    k: usize,
) -> Vec<usize> {
    let mut current_route_nodes: Vec<usize> = Vec::new();

    let mut current_node = instance.vehicles[k].start_node_id;
    let vehicle_end_node = instance.vehicles[k].end_node_id;
    current_route_nodes.push(current_node);

    let mut found_next;
    let num_nodes = instance.nodes.len();

    for _ in 0..num_nodes + 2 {
        found_next = false;

        for next_node in 0..num_nodes {
            if current_node == next_node {
                continue;
            }

            let val = solution.value(variables.x[k][current_node][next_node]);

            if val >= 0.5 {
                current_route_nodes.push(next_node);
                current_node = next_node;
                found_next = true;
                break;
            }
        }

        if !found_next || current_node == vehicle_end_node {
            break;
        }
    }

    current_route_nodes
}
