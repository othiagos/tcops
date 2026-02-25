use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, Default)]
pub enum Metric {
    Euc2d,
    #[default]
    Euc3d,
    Man2d,
    Man3d,
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn distance_to(&self, other: &Point3, metric: &Metric) -> f64 {
        match metric {
            Metric::Euc2d => (
                (self.x - other.x).powi(2) +
                (self.y - other.y).powi(2))
            .sqrt(),
            Metric::Euc3d => (
                (self.x - other.x).powi(2) +
                (self.y - other.y).powi(2) +
                (self.z - other.z).powi(2))
            .sqrt(),
            Metric::Man2d => {
                (self.x - other.x).abs() +
                (self.y - other.y).abs()
            },
            Metric::Man3d => {
                (self.x - other.x).abs() +
                (self.y - other.y).abs() +
                (self.z - other.z).abs()
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub id: usize,
    pub profit: f64,
    pub point: Point3,
    pub parent_subgroup_ids: HashSet<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct Subgroup {
    pub id: usize,
    pub profit: f64,
    pub node_ids: Vec<usize>,
    pub parent_cluster_id: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Cluster {
    pub id: usize,
    pub subgroup_ids: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct Vehicle {
    pub id: usize,
    pub budget: f64,
    pub start_node_id: usize,
    pub end_node_id: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Instance {
    pub name: String,
    pub metric: Metric,
    pub nodes: Vec<Node>,
    pub subgroups: Vec<Subgroup>,
    pub clusters: Vec<Cluster>,
    pub vehicles: Vec<Vehicle>,
}

impl Instance {
    pub fn get_distance(&self, from_id: usize, to_id: usize) -> f64 {
        self.nodes[from_id]
            .point
            .distance_to(&self.nodes[to_id].point, &self.metric)
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
