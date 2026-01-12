use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn distance_to(&self, other: &Point3) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Node {
    pub id: usize,
    pub profit: f64,
    pub point: Point3,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Subgroup {
    pub id: usize,
    pub profit: f64,
    pub node_ids: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cluster {
    pub id: usize,
    pub subgroup_ids: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vehicle {
    pub id: usize,
    pub tmax: f64,
    pub start_node_id: usize,
    pub end_node_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Instance {
    pub name: String,
    pub nodes: Vec<Node>,
    pub subgroups: Vec<Subgroup>,
    pub clusters: Vec<Cluster>,
    pub vehicles: Vec<Vehicle>,
}

impl Instance {
    pub fn get_distance(&self, from_id: usize, to_id: usize) -> f64 {
        self.nodes[from_id]
            .point
            .distance_to(&self.nodes[to_id].point)
    }

    pub fn get_node(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    pub fn get_subgroup(&self, id: usize) -> &Subgroup {
        &self.subgroups[id]
    }

    pub fn get_cluster(&self, id: usize) -> &Cluster {
        &self.clusters[id]
    }
}

pub trait HasId {
    fn id(&self) -> usize;
}

impl HasId for Node {
    fn id(&self) -> usize {
        self.id
    }
}
impl HasId for Subgroup {
    fn id(&self) -> usize {
        self.id
    }
}
impl HasId for Cluster {
    fn id(&self) -> usize {
        self.id
    }
}
impl HasId for Vehicle {
    fn id(&self) -> usize {
        self.id
    }
}
