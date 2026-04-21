use crate::common::instance::{Instance};


pub fn link_parent_references(instance: &mut Instance) {
    link_nodes_references(instance);
    link_clusters_references(instance);
}

fn link_nodes_references(instance: &mut Instance) {
    for subgroup in &mut instance.subgroups {
        for node_id in &subgroup.node_ids {
            instance.nodes[*node_id]
                .parent_subgroup_ids
                .insert(subgroup.id);
        }
    }
}

fn link_clusters_references(instance: &mut Instance) {
    for cluster in &instance.clusters {
        for subgroup_id in &cluster.subgroup_ids {
            instance.subgroups[*subgroup_id].parent_cluster_id = cluster.id;
        }
    }
}