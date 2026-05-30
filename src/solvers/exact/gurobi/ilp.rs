#![allow(clippy::useless_conversion)]

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
        if let Some(limit) = args.time_limit {
            self.model.set_param(param::TimeLimit, limit as f64).map_err(Self::map_err)?;
        }

        self.model.set_param(param::LazyConstraints, 1).map_err(Self::map_err)?;

        constraint::flow_conservation(&mut self.model, &self.variables, self.instance).map_err(Self::map_err)?;
        constraint::unique_visit(&mut self.model, &self.variables, self.instance).map_err(Self::map_err)?;
        constraint::logical_physical(&mut self.model, &self.variables, self.instance).map_err(Self::map_err)?;
        constraint::cluster(&mut self.model, &self.variables, self.instance).map_err(Self::map_err)?;
        constraint::budget(&mut self.model, &self.variables, self.instance).map_err(Self::map_err)?;
        
        objective::set_objective(&mut self.model, &self.variables, self.instance).map_err(Self::map_err)?;

        let mut callback = SubtourCallback::new(&self.variables, self.instance);

        self.model.optimize_with_callback(&mut callback).map_err(Self::map_err)?;
        let status = self.model.status().map_err(Self::map_err)?;
        
        match status {
            Status::Optimal | Status::TimeLimit | Status::IterationLimit => {
                let metrics = SolverMetrics {
                    best_bound: self.model.get_attr(attr::ObjBound).ok(),
                    gap: self.model.get_attr(attr::MIPGap).ok(),
                    explored_nodes: self.model.get_attr(attr::NodeCount).ok().map(|n| n as u64),
                };

                parser::parse_solution(&self.model, &self.variables, self.instance, start_time.elapsed(), metrics).map_err(|e| {
                    SolverError::new(
                        SolverErrorKind::Parser,
                        &format!("Error parsing Gurobi results: {}", e),
                    )
                })
            }
            _ => Err(SolverError::new(
                SolverErrorKind::Solver,
                &format!("Gurobi failed to find a solution. Status: {:?}", status),
            )),
        }
    }

    fn map_err(e: grb::Error) -> SolverError {
        SolverError::new(
            SolverErrorKind::Solver,
            &format!("Gurobi Error: {}", e),
        )
    }
}