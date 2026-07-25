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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_status_display() {
        assert_eq!(SolutionStatus::Optimal.to_string(), "Optimal");
        assert_eq!(SolutionStatus::Feasible.to_string(), "Feasible");
        assert_eq!(SolutionStatus::TimeLimit.to_string(), "Time Limit");
        assert_eq!(SolutionStatus::IterationLimit.to_string(), "Iteration Limit");
        assert_eq!(SolutionStatus::Unknown.to_string(), "Unknown");
        assert_eq!(SolutionStatus::default(), SolutionStatus::Unknown);
    }

    #[test]
    fn test_solution_status_from_grb_status() {
        assert_eq!(SolutionStatus::from(Status::Optimal), SolutionStatus::Optimal);
        assert_eq!(SolutionStatus::from(Status::TimeLimit), SolutionStatus::TimeLimit);
        assert_eq!(SolutionStatus::from(Status::IterationLimit), SolutionStatus::IterationLimit);
        assert_eq!(SolutionStatus::from(Status::Infeasible), SolutionStatus::Unknown);
    }

    #[test]
    fn test_solution_get_objective_value() {
        let instance = Instance::default();
        let solution = Solution {
            instance: &instance,
            duration: Duration::from_secs(1),
            routes: vec![],
            total_cost: 15.5,
            total_score: 100.0,
            status: SolutionStatus::Optimal,
            solver: Some("exact".to_string()),
            best_bound: Some(100.0),
            gap: Some(0.0),
            explored_nodes: Some(10),
        };

        assert_eq!(solution.get_objective_value(), 100.0);
    }
}

