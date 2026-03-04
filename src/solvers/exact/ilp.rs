use good_lp::Solver;
use good_lp::{Constraint, Expression, ProblemVariables, SolverModel, Variable, variables};

use crate::common::{
    error::{SolverError, SolverErrorKind},
    instance::Instance,
    solution::Solution,
};

use crate::solvers::exact::{constraint, objective, parser, variable};

pub struct DecisionVariables {
    pub x: Vec<Vec<Vec<Variable>>>,
    pub y: Vec<Vec<Variable>>,
    pub z: Vec<Variable>,
    pub w: Vec<Variable>,
    pub u: Vec<Vec<Variable>>,
}

pub struct Ilp<'a> {
    vars: ProblemVariables,
    constraints: Vec<Constraint>,
    objective: Expression,
    variables: DecisionVariables,
    instance: &'a Instance,
}

impl<'a> Ilp<'a> {
    pub fn new(instance: &'a Instance) -> Self {
        let mut vars = variables!();

        let x = variable::initialize_x(instance, &mut vars);
        let y = variable::initialize_y(instance, &mut vars);
        let z = variable::initialize_z(instance, &mut vars);
        let w = variable::initialize_w(instance, &mut vars);
        let u = variable::initialize_u(instance, &mut vars);

        let variables = DecisionVariables { x, y, z, w, u };
        let objective = objective::function(&variables, instance);
        let constraints = Self::set_constraints(&variables, instance);

        Self {
            vars,
            constraints,
            objective,
            variables,
            instance,
        }
    }

    fn set_constraints(variable: &DecisionVariables, instance: &Instance) -> Vec<Constraint> {
        let mut constraints = Vec::new();

        constraints.extend(constraint::flow_conservation(variable, instance));
        constraints.extend(constraint::unique_visit(variable, instance));
        constraints.extend(constraint::logical_physical(variable, instance));
        constraints.extend(constraint::cluster(variable, instance));
        constraints.extend(constraint::budget(variable, instance));
        constraints.extend(constraint::subtour_elimination_mtz(variable, instance));

        constraints
    }

    pub fn solve<S, M, F>(self, solver: S, configure: F) -> Result<Solution<'a>, SolverError>
    where
        S: Solver,
        S::Model: SolverModel,
        M: SolverModel,
        F: FnOnce(S::Model) -> M,
    {
        let Ilp {
            vars,
            constraints,
            objective,
            variables,
            instance,
        } = self;

        let base_model = vars
            .maximise(&objective)
            .using(solver)
            .with_all(constraints);

        let configured_model = configure(base_model);

        match configured_model.solve() {
            Ok(solution) => Ok(parser::parse_solution(solution, variables, instance)),
            Err(e) => Err(SolverError::new(
                SolverErrorKind::Solver,
                &format!("Error in Solver: {}", e),
            )),
        }
    }
}
