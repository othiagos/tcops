use std::collections::VecDeque;
use super::max_flow::{max_flow_min_cut, Edge};

pub fn find_fractional_subtours(
    num_nodes: usize,
    end_node: usize,
    edges: &[(usize, usize, f64)],
    y_vals: &[f64],
) -> Vec<(Vec<usize>, f64)> {
    let mut old_to_new = vec![usize::MAX; num_nodes];
    let mut new_to_old = Vec::new();

    old_to_new[end_node] = 0;
    new_to_old.push(end_node);

    for &(u, v, _) in edges {
        if old_to_new[u] == usize::MAX {
            old_to_new[u] = new_to_old.len();
            new_to_old.push(u);
        }
        if old_to_new[v] == usize::MAX {
            old_to_new[v] = new_to_old.len();
            new_to_old.push(v);
        }
    }

    let num_active = new_to_old.len();
    let mut adj: Vec<Vec<Edge>> = vec![vec![]; num_active];

    for &(u, v, cap) in edges {
        let nu = old_to_new[u];
        let nv = old_to_new[v];
        let rev_u = adj[nv].len();
        let rev_v = adj[nu].len();
        adj[nu].push(Edge { to: nv, cap, rev_idx: rev_u });
        adj[nv].push(Edge { to: nu, cap: 0.0, rev_idx: rev_v });
    }

    let candidates: Vec<(usize, f64)> = y_vals
        .iter()
        .enumerate()
        .filter(|&(i, &y_val)| i != end_node && y_val > 1e-4 && old_to_new[i] != usize::MAX)
        .map(|(i, &y_val)| (old_to_new[i], y_val))
        .collect();

    let mut all_bad_tours = Vec::new();

    for (src, y_val) in candidates {
        let mut visited = vec![false; num_active];
        let mut queue = VecDeque::new();
        queue.push_back(src);
        visited[src] = true;
        let mut reaches_sink = false;

        while let Some(u) = queue.pop_front() {
            if u == 0 {
                reaches_sink = true;
                break;
            }

            for edge in &adj[u] {
                if edge.cap > 1e-6 && !visited[edge.to] {
                    visited[edge.to] = true;
                    queue.push_back(edge.to);
                }
            }
        }

        if !reaches_sink {
            let mut component: Vec<usize> = (0..num_active)
                .filter(|&i| visited[i])
                .map(|i| new_to_old[i])
                .collect();

            component.sort_unstable();

            let violation = y_val;
            all_bad_tours.push((component, violation));

            continue;
        }

        let mut adj_run = adj.clone();
        let (max_flow, min_cut) = max_flow_min_cut(num_active, &mut adj_run, src, 0);

        let violation = y_val - max_flow;
        if violation > 1e-4 && !min_cut.contains(&0) {
            let mut component: Vec<usize> = min_cut.into_iter().map(|i| new_to_old[i]).collect();
            component.sort_unstable();
            all_bad_tours.push((component, violation));
        }
    }

    all_bad_tours
}

pub fn find_invalid_subtours(
    num_nodes: usize,
    start_node: usize,
    end_node: usize,
    active_edges: &[(usize, usize)],
) -> Vec<Vec<usize>> {
    let mut graph: Vec<Vec<usize>> = vec![vec![]; num_nodes];
    for &(i, j) in active_edges {
        graph[i].push(j);
        graph[j].push(i);
    }

    let mut visited = vec![false; num_nodes];
    let mut bad_tours = Vec::new();

    for node in 0..num_nodes {
        if !visited[node] && !graph[node].is_empty() {
            let mut component = Vec::new();
            let mut stack = vec![node];

            while let Some(current) = stack.pop() {
                if !visited[current] {
                    visited[current] = true;
                    component.push(current);

                    for &neighbor in &graph[current] {
                        stack.push(neighbor);
                    }
                }
            }

            if !component.contains(&start_node) && !component.contains(&end_node) {
                bad_tours.push(component);
            }
        }
    }
    bad_tours
}