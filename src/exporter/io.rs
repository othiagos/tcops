use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;

use serde::Serialize;

use crate::common::instance::Metric;
use crate::common::solution::Solution;

#[derive(Serialize)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
    z: f64,
    parent_subgroup_ids: Vec<usize>,
}

#[derive(Serialize)]
struct Subgroup {
    id: usize,
    profit: f64,
    node_ids: Vec<usize>,
}

#[derive(Serialize)]
struct Cluster {
    id: usize,
    subgroup_ids: Vec<usize>,
}

#[derive(Serialize)]
struct Vehicle {
    id: usize,
    budget: f64,
    start_node_id: usize,
    end_node_id: usize,
}

#[derive(Serialize)]
struct Route {
    vehicle_id: usize,
    path: Vec<usize>,
}

#[derive(Serialize)]
struct JsonSolution {
    instance_name: String,
    mode: String,
    solver: Option<String>,
    elapsed_time_sec: f64,
    status: String,
    total_cost: f64,
    total_score: f64,
    best_bound: Option<f64>,
    gap: Option<f64>,
    explored_nodes: Option<u64>,
    vehicles_used_ids: Vec<usize>,
    clusters_visited_ids: Vec<usize>,
    subgroups_visited_ids: Vec<usize>,
    nodes: Vec<Node>,
    subgroups: Vec<Subgroup>,
    clusters: Vec<Cluster>,
    vehicles: Vec<Vehicle>,
    routes: Vec<Route>,
}

pub fn export_solution_to_json(solution: &Solution) -> Option<String> {
    let instance_name = solution.instance.name.clone();
    let mode = get_mode(solution);
    let elapsed_time_sec = solution.duration.as_secs_f64();
    let total_cost = solution.total_cost;
    let total_score = solution.total_score;
    let vehicles_used_ids = get_vehicles_used(solution);
    let clusters_visited_ids = get_clusters_visited(solution);
    let subgroups_visited_ids = get_subgroups_visited(solution);
    let status = solution.status.clone().to_string();
    let solver = solution.solver.clone();
    let best_bound = solution.best_bound;
    let gap = solution.gap;
    let explored_nodes = solution.explored_nodes;
    let nodes = get_node(solution);
    let subgroups = get_subgroup(solution);
    let clusters = get_cluster(solution);
    let vehicles = get_vehicles(solution);
    let routes = get_routes(solution);

    let json_solution = JsonSolution {
        instance_name,
        mode,
        elapsed_time_sec,
        status,
        total_cost,
        total_score,
        vehicles_used_ids,
        clusters_visited_ids,
        subgroups_visited_ids,
        nodes,
        subgroups,
        clusters,
        vehicles,
        routes,
        solver,
        best_bound,
        gap,
        explored_nodes,
    };

    let path = format!(
        "{}/{}.json",
        solution.instance.folder_path, solution.instance.name
    );

    let file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create JSON file: {}", e);
            return None;
        }
    };

    if let Err(e) = serde_json::to_writer_pretty(BufWriter::new(file), &json_solution) {
        eprintln!("Failed to write JSON: {}", e);
        return None;
    }

    Some(path)
}

fn get_mode(solution: &Solution) -> String {
    match solution.instance.metric {
        Metric::Euc2d | Metric::Man2d => "2d".to_string(),
        Metric::Euc3d | Metric::Man3d => "3d".to_string(),
    }
}

fn get_node(solution: &Solution) -> Vec<Node> {
    solution
        .instance
        .nodes
        .iter()
        .map(|n| Node {
            id: n.id,
            x: n.point.x,
            y: n.point.y,
            z: n.point.z,
            parent_subgroup_ids: n.parent_subgroup_ids.iter().copied().collect(),
        })
        .collect()
}

fn get_subgroup(solution: &Solution) -> Vec<Subgroup> {
    solution
        .instance
        .subgroups
        .iter()
        .map(|s| Subgroup {
            id: s.id,
            profit: s.profit,
            node_ids: s.node_ids.clone(),
        })
        .collect()
}

fn get_cluster(solution: &Solution) -> Vec<Cluster> {
    solution
        .instance
        .clusters
        .iter()
        .map(|c| Cluster {
            id: c.id,
            subgroup_ids: c.subgroup_ids.clone(),
        })
        .collect()
}

fn get_vehicles(solution: &Solution) -> Vec<Vehicle> {
    solution
        .instance
        .vehicles
        .iter()
        .map(|v| Vehicle {
            id: v.id,
            budget: v.budget,
            start_node_id: v.start_node_id,
            end_node_id: v.end_node_id,
        })
        .collect()
}

fn get_routes(solution: &Solution) -> Vec<Route> {
    solution.routes.iter().map(|r| Route {
        vehicle_id: r.vehicle_id,
        path: r.path.clone(),
    }).collect()
}

fn get_vehicles_used(solution: &Solution) -> Vec<usize> {
    let mut used_vehicles = HashSet::new();

    for route in &solution.routes {
        if route.path.len() > 2 {
            used_vehicles.insert(route.vehicle_id);
        }
    }

    used_vehicles.into_iter().collect()
}

fn get_clusters_visited(solution: &Solution) -> Vec<usize> {
    let mut visited_clusters = HashSet::new();

    for route in &solution.routes {
        for &node_id in &route.path {
            let node = &solution.instance.nodes[node_id];
            for &sg_id in &node.parent_subgroup_ids {
                let c_id = solution.instance.subgroups[sg_id].parent_cluster_id;
                visited_clusters.insert(c_id);
            }
        }
    }

    visited_clusters.into_iter().collect()
}

fn get_subgroups_visited(solution: &Solution) -> Vec<usize> {
    let mut visited_subgroups = HashSet::new();

    for route in &solution.routes {
        for &node_id in &route.path {
            let node = &solution.instance.nodes[node_id];
            for &sg_id in &node.parent_subgroup_ids {
                visited_subgroups.insert(sg_id);
            }
        }
    }

    visited_subgroups.into_iter().collect()
}