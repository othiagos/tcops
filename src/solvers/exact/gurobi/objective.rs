use grb::prelude::*;
use crate::common::instance::Instance;

use crate::solvers::exact::gurobi::ilp::DecisionVariables;

pub fn set_objective(
    model: &mut Model,
    variable: &DecisionVariables,
    instance: &Instance,
) -> grb::Result<()> {
    
    let obj_expr: grb::Expr = (0..instance.subgroups.len())
        .map(|s| instance.subgroups[s].profit * variable.z[s])
        .sum();

    model.set_objective(obj_expr, Maximize)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::instance::{Cluster, Node, Subgroup, Vehicle};
    use crate::solvers::exact::gurobi::variable;

    #[test]
    fn test_set_objective() -> grb::Result<()> {
        let env = Env::new("gurobi.log")?;
        let mut model = Model::with_env("obj_test", &env)?;

        let instance = Instance {
            nodes: vec![
                Node { id: 0, ..Default::default() },
                Node { id: 1, ..Default::default() },
            ],
            subgroups: vec![
                Subgroup { id: 0, profit: 15.5, node_ids: vec![1], ..Default::default() },
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
        set_objective(&mut model, &vars, &instance)?;

        model.update()?;
        assert!(matches!(model.get_attr(attr::ModelSense)?, ModelSense::Maximize));

        Ok(())
    }
}