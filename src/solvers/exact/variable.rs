use crate::common::instance::Instance;

use good_lp::{ProblemVariables, Variable, VariableDefinition, variable};

pub fn initialize_x(instance: &Instance, vars: &mut ProblemVariables) -> Vec<Vec<Vec<Variable>>> {
    let mut x = vec![
        vec![
            vec![vars.add(VariableDefinition::default()); instance.nodes.len()];
            instance.nodes.len()
        ];
        instance.vehicles.len()
    ];

    for (k, x_k) in x.iter_mut().enumerate() {
        for (i, x_ki) in x_k.iter_mut().enumerate() {
            for (j, x_kij) in x_ki.iter_mut().enumerate() {
                if i != j {
                    *x_kij = vars.add(variable().binary().name(format!("x_{}_{}_{}", k, i, j)))
                }
            }
        }
    }

    x
}

pub fn initialize_y(instance: &Instance, vars: &mut ProblemVariables) -> Vec<Vec<Variable>> {
    let mut y = vec![
        vec![vars.add(VariableDefinition::default()); instance.nodes.len()];
        instance.vehicles.len()
    ];

    for (k, y_k) in y.iter_mut().enumerate() {
        for (i, y_ki) in y_k.iter_mut().enumerate() {
            *y_ki = vars.add(variable().binary().name(format!("y_{}_{}", k, i)));
        }
    }

    y
}

pub fn initialize_z(instance: &Instance, vars: &mut ProblemVariables) -> Vec<Variable> {
    let mut z = Vec::new();

    for s in 0..instance.subgroups.len() {
        z.push(vars.add(variable().binary().name(format!("z_{}", s))));
    }

    z
}

pub fn initialize_w(instance: &Instance, vars: &mut ProblemVariables) -> Vec<Variable> {
    let mut w = Vec::new();

    for c in 0..instance.clusters.len() {
        w.push(vars.add(variable().binary().name(format!("w_{}", c))));
    }

    w
}

pub fn initialize_u(instance: &Instance, vars: &mut ProblemVariables) -> Vec<Vec<Variable>> {
    let num_nodes = instance.nodes.len();
    let num_vehicles = instance.vehicles.len();
    let mut u = vec![vec![vars.add(VariableDefinition::default()); num_nodes]; num_vehicles];

    for (k, u_k) in u.iter_mut().enumerate() {
        for (i, u_ki) in u_k.iter_mut().enumerate() {
            // U ranges from 1 to N (number of nodes)
            *u_ki = vars.add(
                variable()
                    .min(1.0)
                    .max(num_nodes as f64)
                    .name(format!("u_{}_{}", k, i)),
            );
        }
    }

    u
}
