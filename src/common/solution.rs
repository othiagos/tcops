#[derive(Debug, Clone, PartialEq, Default)]
pub enum SolutionStatus {
    Optimal,
    Feasible,
    Infeasible,
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
    pub routes: Vec<Route>,
    pub total_cost: f64,
    pub total_score: f64,
    pub status: SolutionStatus,
}
