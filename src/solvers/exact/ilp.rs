use crate::common::{
    error::{SolverError, SolverErrorKind},
    instance::Instance,
    solution::Solution,
};

use good_lp::{
    Constraint, Expression, LpSolver, ProblemVariables, SolverModel, Variable,
    solvers::lp_solvers::{GurobiSolver, LpSolution},
    variable, variables,
};

struct UsedVariables {
    x: Variable,
    y: Variable,
}

pub struct Ilp {
    vars: ProblemVariables,
    constraints: Vec<Constraint>,
    objective: Expression,
}

impl Ilp {
    pub fn new(instance: Instance) -> Self {
        let mut vars = variables!();

        let x = vars.add(variable().min(0).name("var_x"));
        let y = vars.add(variable().min(0).name("var_y"));

        let objective = 2.0 * x + y;
        let variables = UsedVariables { x, y };
        let constraints = Self::set_constraints(variables, &instance);

        Self {
            vars,
            constraints,
            objective,
        }
    }

    fn set_constraints(variables: UsedVariables, instance: &Instance) -> Vec<Constraint> {
        vec![
            Self::rule1(&variables, instance),
            Self::rule2(&variables, instance),
        ]
    }

    fn rule1(variables: &UsedVariables, instance: &Instance) -> Constraint {
        (variables.x + variables.y) << instance.subgroups.len() as i32
    }

    fn rule2(variables: &UsedVariables, instance: &Instance) -> Constraint {
        variables.x << instance.nodes.len() as i32
    }

    pub fn solve(self) -> Result<Solution, SolverError> {
        let problem_base = self.vars.maximise(self.objective);
        let solver = LpSolver(GurobiSolver::new());
        let model = problem_base.using(solver);

        let solution = model.with_all(self.constraints).solve();
        match solution {
            Ok(sol) => Ok(parse_solution(sol)),
            Err(e) => Err(SolverError::new(
                SolverErrorKind::GurobiSolverError,
                &format!("Gurobi is not available on the OS: {}", e),
            )),
        }
    }
}

fn parse_solution(_solution: LpSolution) -> Solution {
    Solution::default()
}
