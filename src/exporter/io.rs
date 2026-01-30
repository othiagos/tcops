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
struct JsonSolution {
    mode: String,
    nodes: Vec<Node>,
    subgroups: Vec<Subgroup>,
    clusters: Vec<Cluster>,
    routes: Vec<Vec<usize>>,
}

pub fn export_solution_to_json(path: &str, solution: &Solution) {
    let mode = get_mode(solution);
    let nodes = get_node(solution);
    let subgroups = get_subgroup(solution);
    let clusters = get_cluster(solution);
    let routes = get_routes(solution);

    let json_solution = JsonSolution {
        mode,
        nodes,
        subgroups,
        clusters,
        routes,
    };

    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create JSON file: {}", e);
            return;
        }
    };

    if let Err(e) = serde_json::to_writer_pretty(BufWriter::new(file), &json_solution) {
        eprintln!("Failed to write JSON: {}", e)
    }
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

fn get_routes(solution: &Solution) -> Vec<Vec<usize>> {
    solution.routes.iter().map(|r| r.path.clone()).collect()
}
