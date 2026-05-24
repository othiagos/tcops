use crate::common::instance::Instance;
use crate::solvers::exact::gurobi::ilp::DecisionVariables;

pub struct CallbackCache {
    pub x_vars: Vec<Vec<grb::Var>>,
    pub x_edges: Vec<Vec<(usize, usize)>>,
    pub y_vars: Vec<Vec<grb::Var>>,
}

impl CallbackCache {
    pub fn new(variables: &DecisionVariables, instance: &Instance) -> Self {
        let num_vehicles = instance.vehicles.len();
        let num_nodes = instance.nodes.len();

        let mut x_vars = Vec::with_capacity(num_vehicles);
        let mut x_edges = Vec::with_capacity(num_vehicles);
        let mut y_vars = Vec::with_capacity(num_vehicles);

        for k in 0..num_vehicles {
            let mut x_v = Vec::with_capacity(num_nodes * num_nodes);
            let mut x_e = Vec::with_capacity(num_nodes * num_nodes);
            let mut y_v = Vec::with_capacity(num_nodes);

            for i in 0..num_nodes {
                y_v.push(variables.y[k][i]);
                for j in 0..num_nodes {
                    if i != j {
                        x_v.push(variables.x[k][i][j]);
                        x_e.push((i, j));
                    }
                }
            }

            x_vars.push(x_v);
            x_edges.push(x_e);
            y_vars.push(y_v);
        }

        Self {
            x_vars,
            x_edges,
            y_vars,
        }
    }
}
