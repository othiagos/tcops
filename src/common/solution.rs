use std::fmt;
use std::time::Duration;

use grb::Status;

use crate::common::instance::Instance;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SolutionStatus {
    Optimal,
    Feasible,
    TimeLimit,
    IterationLimit,
    #[default]
    Unknown,
}

impl From<Status> for SolutionStatus {
    fn from(status: Status) -> Self {
        match status {
            Status::Optimal => SolutionStatus::Optimal,
            Status::TimeLimit => SolutionStatus::TimeLimit,
            Status::IterationLimit => SolutionStatus::IterationLimit,
            _ => SolutionStatus::Unknown,
        }
    }
}

impl fmt::Display for SolutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolutionStatus::Optimal => write!(f, "Optimal"),
            SolutionStatus::Feasible => write!(f, "Feasible"),
            SolutionStatus::TimeLimit => write!(f, "Time Limit"),
            SolutionStatus::IterationLimit => write!(f, "Iteration Limit"),
            SolutionStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Route {
    pub vehicle_id: usize,
    pub path: Vec<usize>,
    pub cost: f64,
}

#[derive(Debug, Clone)]
pub struct Solution<'a> {
    pub instance: &'a Instance,
    pub duration: Duration,
    pub routes: Vec<Route>,
    pub total_cost: f64,
    pub total_score: f64,
    pub status: SolutionStatus,
    pub solver: Option<String>,
    pub best_bound: Option<f64>,
    pub gap: Option<f64>,
    pub explored_nodes: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct SolverMetrics {
    pub best_bound: Option<f64>,
    pub gap: Option<f64>,
    pub explored_nodes: Option<u64>,
}

impl<'a> Solution<'a> {
    pub fn get_objective_value(&self) -> f64 {
        self.total_score
    }
}
