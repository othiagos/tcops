use std::fmt;
use std::time::Duration;

use crate::common::instance::Instance;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SolutionStatus {
    Optimal,
    Feasible,
    #[default]
    Unknown,
}

impl fmt::Display for SolutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolutionStatus::Optimal => write!(f, "Optimal"),
            SolutionStatus::Feasible => write!(f, "Feasible"),
            SolutionStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Route {
    pub vehicle_id: usize,
    pub path: Vec<usize>,
    pub cost: f64,
    pub score: f64,
}

#[derive(Debug, Clone)]
pub struct Solution<'a> {
    pub instance: &'a Instance,
    pub duration: Duration,
    pub routes: Vec<Route>,
    pub total_cost: f64,
    pub total_score: f64,
    pub status: SolutionStatus,
}

impl<'a> Solution<'a> {
    pub fn get_objective_value(&self) -> f64 {
        self.total_score
    }
}
