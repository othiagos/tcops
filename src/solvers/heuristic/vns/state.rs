use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct SearchState {
    pub visited_nodes: HashSet<usize>,
    pub cluster_locks: HashMap<usize, usize>,
    pub subgroup_nodes_count: HashMap<usize, usize>,
}
