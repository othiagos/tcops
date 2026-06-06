use std::time::Instant;

use grb::prelude::*;

use crate::{
    common::{
        error::{SolverError, SolverErrorKind},
        instance::Instance,
        solution::{Solution, SolverMetrics},
    },
    solvers::exact::gurobi::{
        callback::subtour::SubtourCallback,
        constraint,
        objective, 
        parser,
        variable
    }
};

use crate::cli::Cli;
pub use variable::DecisionVariables;

pub struct Ilp<'a> {
    model: Model,
    variables: DecisionVariables,
    instance: &'a Instance,
}

impl<'a> Ilp<'a> {
    pub fn new(instance: &'a Instance) -> grb::Result<Self> {
        let env = Env::new("gurobi.log")?;
        let mut model = Model::with_env(&instance.name, &env)?;

        let x = variable::initialize_x(&mut model, instance)?;
        let y = variable::initialize_y(&mut model, instance)?;
        let z = variable::initialize_z(&mut model, instance)?;
        let w = variable::initialize_w(&mut model, instance)?;

        let variables = DecisionVariables { x, y, z, w };

        Ok(Self {
            model,
            variables,
            instance,
        })
    }

    pub fn solve(mut self, args: &Cli) -> Result<Solution<'a>, SolverError> {
        let start_time = Instant::now();

        self.configure_solver(args)?;

        self.build_model()?;

        let mut callback = SubtourCallback::new(&self.variables, self.instance);
        self.model.optimize_with_callback(&mut callback)?;

        self.extract_solution(start_time)
    }

    fn configure_solver(&mut self, args: &Cli) -> Result<(), SolverError> {
        if let Some(limit) = args.time_limit {
            self.model.set_param(param::TimeLimit, limit as f64)?;
        }

        self.model.set_param(param::LazyConstraints, 1)?;
        self.model.get_env_mut().read_params("gurobi.prm")?;

        Ok(())
    }

    fn build_model(&mut self) -> Result<(), SolverError> {
        let m = &mut self.model;
        let v = &self.variables;
        let i = self.instance;

        constraint::flow_conservation(m, v, i)?;
        constraint::unique_visit(m, v, i)?;
        constraint::logical_physical(m, v, i)?;
        constraint::cluster(m, v, i)?;
        constraint::budget(m, v, i)?;

        objective::set_objective(m, v, i)?;

        Ok(())
    }

    fn extract_solution(&self, start_time: Instant) -> Result<Solution<'a>, SolverError> {
        let status = self.model.status()?;

        match status {
            Status::Optimal | Status::TimeLimit | Status::IterationLimit => {
                self.handle_successful_solution(start_time, status)
            }
            _ => {
                self.handle_failed_solution(status)
            }
        }
    }

    fn handle_successful_solution(&self, start_time: Instant, status: Status) -> Result<Solution<'a>, SolverError> {
        let metrics = SolverMetrics {
            best_bound: self.model.get_attr(attr::ObjBound).ok(),
            gap: self.model.get_attr(attr::MIPGap).ok(),
            explored_nodes: self.model.get_attr(attr::NodeCount).ok().map(|n| n as u64),
        };

        parser::parse_solution(
            &self.model,
            &self.variables,
            self.instance,
            start_time.elapsed(),
            metrics,
            status,
        )
        .map_err(|e| {
            SolverError::new(
                SolverErrorKind::Parser,
                &format!("Error parsing Gurobi results: {}", e),
            )
        })
    }

    fn handle_failed_solution(&self, status: Status) -> Result<Solution<'a>, SolverError> {
        Err(SolverError::new(
            SolverErrorKind::Solver,
            &format!("Gurobi failed to find a solution. Status: {:?}", status),
        ))
    }
}
