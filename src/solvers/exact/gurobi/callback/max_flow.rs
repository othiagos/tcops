use std::collections::VecDeque;

#[derive(Clone)]
pub struct Edge {
    pub to: usize,
    pub cap: f64,
    pub rev_idx: usize,
}

pub fn max_flow_min_cut(
    num_active_nodes: usize,
    adj: &mut [Vec<Edge>],
    source: usize,
    sink: usize,
) -> (f64, Vec<usize>) {
    let mut max_flow = 0.0;
    let eps = 1e-6;

    loop {
        let mut parent = vec![(usize::MAX, usize::MAX); num_active_nodes];
        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(u) = queue.pop_front() {
            if u == sink {
                break;
            }

            for (idx, edge) in adj[u].iter().enumerate() {
                if parent[edge.to].0 == usize::MAX && edge.to != source && edge.cap > eps {
                    parent[edge.to] = (u, idx);
                    queue.push_back(edge.to);
                }
            }
        }

        if parent[sink].0 == usize::MAX {
            break;
        }

        let mut push_flow = f64::MAX;
        let mut curr = sink;
        while curr != source {
            let (p, idx) = parent[curr];
            push_flow = push_flow.min(adj[p][idx].cap);
            curr = p;
        }

        let mut curr = sink;
        while curr != source {
            let (p, idx) = parent[curr];
            let rev_idx = adj[p][idx].rev_idx;
            adj[p][idx].cap -= push_flow;
            adj[curr][rev_idx].cap += push_flow;
            curr = p;
        }
        max_flow += push_flow;
    }

    let mut visited = vec![false; num_active_nodes];
    let mut queue = VecDeque::new();
    queue.push_back(source);
    visited[source] = true;

    while let Some(u) = queue.pop_front() {
        for edge in &adj[u] {
            if !visited[edge.to] && edge.cap > eps {
                visited[edge.to] = true;
                queue.push_back(edge.to);
            }
        }
    }

    let min_cut_component: Vec<usize> = (0..num_active_nodes).filter(|&i| visited[i]).collect();
    (max_flow, min_cut_component)
}
