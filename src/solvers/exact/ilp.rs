use good_lp::{
    Constraint, Expression, LpSolver, ProblemVariables, SolverModel, Variable,
    solvers::lp_solvers::GurobiSolver, variables,
};

use crate::common::{
    error::{SolverError, SolverErrorKind},
    instance::Instance,
    solution::Solution,
};

use crate::solvers::exact::{constraint, objective, parser, variable};

pub struct UsedVariables {
    pub x: Vec<Vec<Vec<Variable>>>,
    pub y: Vec<Vec<Variable>>,
    pub z: Vec<Variable>,
    pub w: Vec<Variable>,
    pub u: Vec<Vec<Variable>>,
}

pub struct Ilp {
    vars: ProblemVariables,
    constraints: Vec<Constraint>,
    objective: Expression,
    variables: UsedVariables,
    instance: Instance,
}

impl Ilp {
    pub fn new(instance: Instance) -> Self {
        let mut vars = variables!();

        let x = variable::initialize_x(&instance, &mut vars);
        let y = variable::initialize_y(&instance, &mut vars);
        let z = variable::initialize_z(&instance, &mut vars);
        let w = variable::initialize_w(&instance, &mut vars);
        let u = variable::initialize_u(&instance, &mut vars);

        let variables = UsedVariables { x, y, z, w, u };
        let objective = objective::function(&variables, &instance);
        let constraints = Self::set_constraints(&variables, &instance);

        println!(
            "ILP Model initialized with {} variables and {} constraints.",
            vars.len(),
            constraints.len()
        );

        Self {
            vars,
            constraints,
            objective,
            variables,
            instance,
        }
    }

    fn set_constraints(variable: &UsedVariables, instance: &Instance) -> Vec<Constraint> {
        let mut constraints = Vec::new();

        constraints.extend(constraint::flow_conservation(variable, instance));
        constraints.extend(constraint::unique_visit(variable, instance));
        constraints.extend(constraint::logical_physical(variable, instance));
        constraints.extend(constraint::cluster(variable, instance));
        constraints.extend(constraint::budget(variable, instance));
        constraints.extend(constraint::subtour_elimination_mtz(variable, instance));
        constraints
    }

    pub fn solve(self) -> Result<Solution, SolverError> {
        let Ilp {
            vars,
            constraints,
            objective,
            variables,
            instance,
        } = self;

        let objective_for_eval = objective.clone();

        let problem = vars.maximise(objective);
        let solver = LpSolver(GurobiSolver::new());
        let model = problem.using(solver).with_all(constraints);

        match model.solve() {
            Ok(solution) => Ok(parser::parse_solution(
                solution,
                variables,
                objective_for_eval,
                instance,
            )),
            Err(e) => Err(SolverError::new(
                SolverErrorKind::GurobiSolverError,
                &format!("Error in Solver Gurubi: {}", e),
            )),
        }
    }
}
