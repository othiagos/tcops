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

#[cfg(test)]
mod tests {
    use super::*;
    use grb::prelude::*;
    use crate::common::instance::{Node, Subgroup, Cluster, Vehicle};
    use crate::solvers::exact::gurobi::variable;

    #[test]
    fn test_callback_cache_new() -> grb::Result<()> {
        let env = Env::new("gurobi.log")?;
        let mut model = Model::with_env("cache_test", &env)?;

        let instance = Instance {
            nodes: vec![
                Node { id: 0, ..Default::default() },
                Node { id: 1, ..Default::default() },
            ],
            subgroups: vec![
                Subgroup { id: 0, profit: 10.0, node_ids: vec![1], ..Default::default() },
            ],
            clusters: vec![
                Cluster { id: 0, subgroup_ids: vec![0] },
            ],
            vehicles: vec![
                Vehicle { id: 0, budget: 10.0, start_node_id: 0, end_node_id: 0 },
            ],
            ..Default::default()
        };

        let x = variable::initialize_x(&mut model, &instance)?;
        let y = variable::initialize_y(&mut model, &instance)?;
        let z = variable::initialize_z(&mut model, &instance)?;
        let w = variable::initialize_w(&mut model, &instance)?;

        let vars = DecisionVariables { x, y, z, w };
        let cache = CallbackCache::new(&vars, &instance);

        assert_eq!(cache.y_vars.len(), 1);
        assert_eq!(cache.y_vars[0].len(), 2);
        assert_eq!(cache.x_vars.len(), 1);
        assert_eq!(cache.x_vars[0].len(), 2);
        assert_eq!(cache.x_edges[0], vec![(0, 1), (1, 0)]);

        Ok(())
    }
}
