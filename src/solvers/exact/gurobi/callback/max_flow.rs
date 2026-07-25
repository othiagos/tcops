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

    let isolated_island: Vec<usize> = (0..num_active_nodes).filter(|&i| !visited[i]).collect();

    (max_flow, isolated_island)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_edge(adj: &mut [Vec<Edge>], from: usize, to: usize, cap: f64) {
        let rev_from = adj[to].len();
        let rev_to = adj[from].len();
        adj[from].push(Edge { to, cap, rev_idx: rev_from });
        adj[to].push(Edge { to: from, cap: 0.0, rev_idx: rev_to });
    }

    #[test]
    fn test_max_flow_simple_path() {
        // 0 -> 1 -> 2
        let num_nodes = 3;
        let mut adj = vec![vec![]; num_nodes];
        add_edge(&mut adj, 0, 1, 5.0);
        add_edge(&mut adj, 1, 2, 3.0);

        let (flow, min_cut) = max_flow_min_cut(num_nodes, &mut adj, 0, 2);
        assert_eq!(flow, 3.0);
        assert_eq!(min_cut, vec![2]);
    }

    #[test]
    fn test_max_flow_disconnected() {
        // 0 (source), 1 (sink) disconnected
        let num_nodes = 2;
        let mut adj = vec![vec![]; num_nodes];

        let (flow, min_cut) = max_flow_min_cut(num_nodes, &mut adj, 0, 1);
        assert_eq!(flow, 0.0);
        assert_eq!(min_cut, vec![1]);
    }

    #[test]
    fn test_max_flow_multiple_paths() {
        // 0 -> 1 -> 3 (cap 2.0, 2.0)
        // 0 -> 2 -> 3 (cap 4.0, 4.0)
        let num_nodes = 4;
        let mut adj = vec![vec![]; num_nodes];
        add_edge(&mut adj, 0, 1, 2.0);
        add_edge(&mut adj, 1, 3, 2.0);
        add_edge(&mut adj, 0, 2, 4.0);
        add_edge(&mut adj, 2, 3, 4.0);

        let (flow, _min_cut) = max_flow_min_cut(num_nodes, &mut adj, 0, 3);
        assert_eq!(flow, 6.0);
    }
}
