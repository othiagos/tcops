use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct SearchState {
    pub visited_nodes: HashSet<usize>,
    pub cluster_locks: HashMap<usize, usize>,
    pub subgroup_nodes_count: HashMap<usize, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_state_default() {
        let state = SearchState::default();
        assert!(state.visited_nodes.is_empty());
        assert!(state.cluster_locks.is_empty());
        assert!(state.subgroup_nodes_count.is_empty());
    }

    #[test]
    fn test_search_state_clone() {
        let mut state = SearchState::default();
        state.visited_nodes.insert(1);
        state.cluster_locks.insert(0, 2);
        state.subgroup_nodes_count.insert(2, 3);

        let cloned = state.clone();
        assert_eq!(cloned.visited_nodes.len(), 1);
        assert!(cloned.visited_nodes.contains(&1));
        assert_eq!(cloned.cluster_locks.get(&0), Some(&2));
        assert_eq!(cloned.subgroup_nodes_count.get(&2), Some(&3));
    }
}
