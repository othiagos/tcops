use crate::common::instance::Instance;
use grb::prelude::*;

pub struct DecisionVariables {
    pub x: Vec<Vec<Vec<Var>>>,
    pub y: Vec<Vec<Var>>,
    pub z: Vec<Var>,
    pub w: Vec<Var>,
}

pub fn initialize_x(model: &mut Model, instance: &Instance) -> grb::Result<Vec<Vec<Vec<Var>>>> {
    let num_vehicles = instance.vehicles.len();
    let num_nodes = instance.nodes.len();

    let mut x = Vec::with_capacity(num_vehicles);

    for k in 0..num_vehicles {
        let mut x_k = Vec::with_capacity(num_nodes);
        for i in 0..num_nodes {
            let mut x_ki = Vec::with_capacity(num_nodes);
            for j in 0..num_nodes {
                if i != j {
                    let var = add_binvar!(model, name: &format!("x_{}_{}_{}", k, i, j))?;
                    x_ki.push(var);
                } else {
                    let var = add_var!(model, Binary, name: &format!("x_diag_{}_{}_{}", k, i, j), bounds: 0.0_f32..0.0_f32)?;
                    x_ki.push(var);
                }
            }
            x_k.push(x_ki);
        }
        x.push(x_k);
    }

    Ok(x)
}

pub fn initialize_y(model: &mut Model, instance: &Instance) -> grb::Result<Vec<Vec<Var>>> {
    let mut y = Vec::with_capacity(instance.vehicles.len());

    for k in 0..instance.vehicles.len() {
        let mut y_k = Vec::with_capacity(instance.nodes.len());
        for i in 0..instance.nodes.len() {
            let var = add_binvar!(model, name: &format!("y_{}_{}", k, i))?;
            y_k.push(var);
        }
        y.push(y_k);
    }

    Ok(y)
}

pub fn initialize_z(model: &mut Model, instance: &Instance) -> grb::Result<Vec<Var>> {
    let mut z = Vec::with_capacity(instance.subgroups.len());

    for s in 0..instance.subgroups.len() {
        let var = add_binvar!(model, name: &format!("z_{}", s))?;
        z.push(var);
    }

    Ok(z)
}

pub fn initialize_w(model: &mut Model, instance: &Instance) -> grb::Result<Vec<Var>> {
    let mut w = Vec::with_capacity(instance.clusters.len());

    for c in 0..instance.clusters.len() {
        let var = add_binvar!(model, name: &format!("w_{}", c))?;
        w.push(var);
    }

    Ok(w)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::instance::{Cluster, Node, Subgroup, Vehicle};

    fn create_test_instance() -> Instance {
        Instance {
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
        }
    }

    #[test]
    fn test_variable_initialization() -> grb::Result<()> {
        let env = Env::new("gurobi.log")?;
        let mut model = Model::with_env("var_test", &env)?;
        let instance = create_test_instance();

        let x = initialize_x(&mut model, &instance)?;
        let y = initialize_y(&mut model, &instance)?;
        let z = initialize_z(&mut model, &instance)?;
        let w = initialize_w(&mut model, &instance)?;

        assert_eq!(x.len(), 1); // 1 vehicle
        assert_eq!(x[0].len(), 2); // 2 nodes
        assert_eq!(x[0][0].len(), 2);

        assert_eq!(y.len(), 1);
        assert_eq!(y[0].len(), 2);

        assert_eq!(z.len(), 1);
        assert_eq!(w.len(), 1);

        Ok(())
    }
}
