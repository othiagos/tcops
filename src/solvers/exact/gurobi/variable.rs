use crate::common::instance::Instance;
use grb::prelude::*;

pub struct DecisionVariables {
    pub x: Vec<Vec<Vec<Var>>>,
    pub y: Vec<Vec<Var>>,
    pub z: Vec<Var>,
    pub w: Vec<Var>,
    pub u: Vec<Vec<Var>>,
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

pub fn initialize_u(model: &mut Model, instance: &Instance) -> grb::Result<Vec<Vec<Var>>> {
    let num_nodes = instance.nodes.len();
    let num_vehicles = instance.vehicles.len();
    let mut u = Vec::with_capacity(num_vehicles);

    for k in 0..num_vehicles {
        let mut u_k = Vec::with_capacity(num_nodes);
        for i in 0..num_nodes {
            let var = add_var!(
                model,
                Continuous,
                name: &format!("u_{}_{}", k, i),
                bounds: 1.0_f32..num_nodes
            )?;
            u_k.push(var);
        }
        u.push(u_k);
    }
    Ok(u)
}
