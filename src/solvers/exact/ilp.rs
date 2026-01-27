use std::collections::HashSet;

use crate::common::{
    error::{SolverError, SolverErrorKind},
    instance::Instance,
    solution::{Route, Solution, SolutionStatus},
};

use good_lp::{
    Constraint, Expression, LpSolver, ProblemVariables, Solution as LpSolverTrait, SolverModel,
    Variable, VariableDefinition,
    solvers::lp_solvers::{GurobiSolver, LpSolution},
    variable, variables,
};

struct UsedVariables {
    x: Vec<Vec<Vec<Variable>>>,
    y: Vec<Vec<Variable>>,
    z: Vec<Variable>,
    w: Vec<Variable>,
    u: Vec<Vec<Variable>>,
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

        let x = Self::initialize_variable_x(&instance, &mut vars);
        let y = Self::initialize_variable_y(&instance, &mut vars);
        let z = Self::initialize_variable_z(&instance, &mut vars);
        let w = Self::initialize_variable_w(&instance, &mut vars);
        let u = Self::initialize_variable_u(&instance, &mut vars);

        let variables = UsedVariables { x, y, z, w, u };
        let objective = Self::objective_function(&variables, &instance);
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

    fn initialize_variable_x(
        instance: &Instance,
        vars: &mut ProblemVariables,
    ) -> Vec<Vec<Vec<Variable>>> {
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

    fn initialize_variable_y(
        instance: &Instance,
        vars: &mut ProblemVariables,
    ) -> Vec<Vec<Variable>> {
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

    fn initialize_variable_z(instance: &Instance, vars: &mut ProblemVariables) -> Vec<Variable> {
        let mut z = Vec::new();

        for s in 0..instance.subgroups.len() {
            z.push(vars.add(variable().binary().name(format!("z_{}", s))));
        }

        z
    }

    fn initialize_variable_w(instance: &Instance, vars: &mut ProblemVariables) -> Vec<Variable> {
        let mut w = Vec::new();

        for c in 0..instance.clusters.len() {
            w.push(vars.add(variable().binary().name(format!("w_{}", c))));
        }

        w
    }

    fn initialize_variable_u(
        instance: &Instance,
        vars: &mut ProblemVariables,
    ) -> Vec<Vec<Variable>> {
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

    fn objective_function(variables: &UsedVariables, instance: &Instance) -> Expression {
        let num_vehicles = instance.vehicles.len();
        let num_nodes = instance.nodes.len();
        let num_subgroups = instance.subgroups.len();

        let mut objective = Expression::from(0.0);
        for s in 0..num_subgroups {
            objective += variables.z[s] * instance.subgroups[s].profit;
        }

        let mut total_dist_expr = Expression::from(0.0);

        for k in 0..num_vehicles {
            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    if i != j {
                        let dist = instance.get_distance(i, j);
                        total_dist_expr += variables.x[k][i][j] * dist;
                    }
                }
            }
        }

        let epsilon = 0.001;
        objective -= total_dist_expr * epsilon;

        objective
    }

    fn set_constraints(variable: &UsedVariables, instance: &Instance) -> Vec<Constraint> {
        let mut constraints = Vec::new();

        constraints.extend(Self::flow_conservation_constraints(variable, instance));
        constraints.extend(Self::unique_visit_constraints(variable, instance));
        constraints.extend(Self::logical_physical_constraints(variable, instance));
        constraints.extend(Self::cluster_constraints(variable, instance));
        constraints.extend(Self::budget_constraints(variable, instance));
        constraints.extend(Self::subtour_elimination_mtz(variable, instance));

        constraints
    }

    fn flow_conservation_constraints(
        variable: &UsedVariables,
        instance: &Instance,
    ) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        let num_nodes = instance.nodes.len();

        for k in 0..instance.vehicles.len() {
            let start_node = instance.vehicles[k].start_node_id;
            let end_node = instance.vehicles[k].end_node_id;

            for i in 0..num_nodes {
                let mut sum_in = Expression::from(0.0);
                for j in 0..num_nodes {
                    if i != j {
                        sum_in += variable.x[k][j][i];
                    }
                }

                let mut sum_out = Expression::from(0.0);
                for j in 0..num_nodes {
                    if i != j {
                        sum_out += variable.x[k][i][j];
                    }
                }
                
                if start_node == end_node {
                    if i == start_node {
                        constraints.push(sum_out.eq(1.0));
                        constraints.push(sum_in.eq(1.0));
                    } else {
                        constraints.push(sum_in.eq(variable.y[k][i]));
                        constraints.push(sum_out.eq(variable.y[k][i]));
                    }
                } 
                else if i == start_node {
                    constraints.push(sum_out.eq(1.0));
                    constraints.push(sum_in.eq(0.0));
                } else if i == end_node {
                    constraints.push(sum_in.eq(1.0));
                    constraints.push(sum_out.eq(0.0));
                } else {
                    constraints.push(sum_in.eq(variable.y[k][i]));
                    constraints.push(sum_out.eq(variable.y[k][i]));
                }
            }
        }
        constraints
    }

    fn unique_visit_constraints(
        variable: &UsedVariables,
        instance: &Instance,
    ) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        let num_nodes = instance.nodes.len();

        let mut depot_nodes = HashSet::new();
        for vehicle in instance.vehicles.iter() {
            depot_nodes.insert(vehicle.start_node_id);
            depot_nodes.insert(vehicle.end_node_id);
        }

        for i in 0..num_nodes {
            if depot_nodes.contains(&i) {
                continue;
            }

            let mut total_visits = Expression::from(0.0);

            for k in 0..instance.vehicles.len() {
                total_visits += variable.y[k][i];
            }

            constraints.push(total_visits.leq(1.0));
        }

        constraints
    }

    fn logical_physical_constraints(
        variable: &UsedVariables,
        instance: &Instance,
    ) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        let num_nodes = instance.nodes.len();

        let mut node_to_subgroups: Vec<Vec<usize>> = vec![Vec::new(); num_nodes];
        for (s_id, subgroup) in instance.subgroups.iter().enumerate() {
            for &node_id in &subgroup.node_ids {
                node_to_subgroups[node_id].push(s_id);
            }
        }

        for (i, subgroups_in_node) in node_to_subgroups.iter().enumerate() {
            let mut sum_z_logic = Expression::from(0.0);
            for &s_id in subgroups_in_node {
                sum_z_logic += variable.z[s_id];
            }

            let mut sum_y_physic = Expression::from(0.0);
            for k in 0..instance.vehicles.len() {
                sum_y_physic += variable.y[k][i];
            }

            constraints.push(sum_z_logic.eq(sum_y_physic));
        }

        constraints
    }

    fn cluster_constraints(
        variable: &UsedVariables,
        instance: &Instance,
    ) -> Vec<Constraint> {
        let mut constraints = Vec::new();

        for (c_id, cluster) in instance.clusters.iter().enumerate() {
            let mut sum_z_subgroups = Expression::from(0.0);

            for &subgroup_id in &cluster.subgroup_ids {
                sum_z_subgroups += variable.z[subgroup_id];
            }

            constraints.push(sum_z_subgroups.eq(variable.w[c_id]));
        }

        constraints
    }

    fn budget_constraints(
        variable: &UsedVariables,
        instance: &Instance,
    ) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        let num_nodes = instance.nodes.len();

        for k in 0..instance.vehicles.len() {
            let mut total_cost_expr = Expression::from(0.0);
            let vehicle_budget = instance.vehicles[k].budget;

            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    if i != j {
                        let dist = instance.get_distance(i, j);
                        total_cost_expr += variable.x[k][i][j] * dist;
                    }
                }
            }

            constraints.push(total_cost_expr.leq(vehicle_budget));
        }

        constraints
    }

    fn subtour_elimination_mtz(
        variable: &UsedVariables,
        instance: &Instance,
    ) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        let n = instance.nodes.len() as f64;

        let mut depot_nodes = HashSet::new();
        for vehicle in instance.vehicles.iter() {
            depot_nodes.insert(vehicle.start_node_id);
            depot_nodes.insert(vehicle.end_node_id);
        }

        for k in 0..instance.vehicles.len() {
            for i in 0..instance.nodes.len() {
                for j in 0..instance.nodes.len() {
                    
                    if depot_nodes.contains(&i) || depot_nodes.contains(&j) {
                        continue;
                    }

                    if i != j {
                        let expr = variable.u[k][i] - variable.u[k][j] + n * variable.x[k][i][j];
                        constraints.push(expr.leq(n - 1.0));
                    }
                }
            }
        }

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
            Ok(solution) => Ok(Self::parse_solution(
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

    fn parse_solution(
        solution: LpSolution,
        variables: UsedVariables,
        objective: Expression,
        instance: Instance,
    ) -> Solution {
        let total_score = LpSolverTrait::eval(&solution, objective);

        let mut routes: Vec<Route> = Vec::new();
        for k in 0..instance.vehicles.len() {
            match Self::get_route(&instance, &solution, &variables, k) {
                Some(route) => routes.push(route),
                None => continue,
            }
        }

        let total_cost = routes.iter().map(|r| r.cost).sum();
        Solution {
            instance,
            total_score,
            total_cost,
            routes,
            status: SolutionStatus::Optimal,
        }
    }

    fn get_route(
        instance: &Instance,
        solution: &LpSolution,
        variables: &UsedVariables,
        k: usize,
    ) -> Option<Route> {
        let current_route_nodes = Self::get_route_node(instance, solution, variables, k);

        if current_route_nodes.is_empty() {
            return None;
        }
 
        let mut route_cost = 0.0;
        let mut route_score = 0.0;

        for i in 0..current_route_nodes.len() - 1 {
            let current_id = current_route_nodes[i];

            route_score += instance.nodes[current_id].profit;

            let next_id = current_route_nodes[i + 1];
            route_cost += instance.get_distance(current_id, next_id);
        }

        let route = Route {
            path: current_route_nodes,
            cost: route_cost,
            score: route_score,
            vehicle_id: k,
        };

        Some(route)
    }

    fn get_route_node(
        instance: &Instance,
        solution: &LpSolution,
        variables: &UsedVariables,
        k: usize,
    ) -> Vec<usize> {
        let mut current_route_nodes: Vec<usize> = Vec::new();

        let mut current_node = instance.vehicles[k].start_node_id;
        let vehicle_end_node = instance.vehicles[k].end_node_id;
        current_route_nodes.push(current_node);

        let mut found_next;
        let num_nodes = instance.nodes.len();

        for _ in 0..num_nodes {
            found_next = false;

            for next_node in 0..num_nodes {
                if current_node == next_node {
                    continue;
                }

                let val = LpSolverTrait::value(solution, variables.x[k][current_node][next_node]);

                if val >= 0.5 {
                    current_route_nodes.push(next_node);
                    current_node = next_node;
                    found_next = true;
                    break;
                }
            }

            if !found_next || current_node == vehicle_end_node {
                break;
            }
        }

        current_route_nodes
    }
}
