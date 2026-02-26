use crate::common::instance::Instance;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SolutionStatus {
    Optimal,
    Feasible,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct Route {
    pub vehicle_id: usize,
    pub path: Vec<usize>,
    pub cost: f64,
    pub score: f64,
}

#[derive(Debug, Clone, Default)]
pub struct Solution {
    pub instance: Instance,
    pub routes: Vec<Route>,
    pub total_cost: f64,
    pub total_score: f64,
    pub status: SolutionStatus,
}

impl Solution {
    pub fn get_objective_value(&self) -> f64 {
        self.total_score - (self.total_cost * 0.001)
    }
}
