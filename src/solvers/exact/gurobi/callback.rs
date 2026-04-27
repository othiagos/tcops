#![allow(clippy::useless_conversion)]

use grb::callback::{Callback, MIPSolCtx, Where};
use grb::prelude::*;

use crate::common::instance::Instance;
use crate::solvers::exact::gurobi::ilp::DecisionVariables;

pub struct SubtourCallback<'a> {
    pub variables: &'a DecisionVariables,
    pub instance: &'a Instance,
}

impl<'a> SubtourCallback<'a> {
    fn get_active_edges(
        &self,
        k: usize,
        ctx: &MIPSolCtx,
    ) -> Result<Vec<(usize, usize)>, anyhow::Error> {
        let num_nodes = self.instance.nodes.len();
        let mut vars_to_query = Vec::with_capacity(num_nodes * num_nodes);
        let mut edge_pairs = Vec::with_capacity(num_nodes * num_nodes);

        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if i != j {
                    vars_to_query.push(self.variables.x[k][i][j]);
                    edge_pairs.push((i, j));
                }
            }
        }

        let solution_vals = ctx.get_solution(vars_to_query.into_iter())?;

        let mut active_edges = Vec::new();
        for (idx, &val) in solution_vals.iter().enumerate() {
            if val > 0.5 {
                active_edges.push(edge_pairs[idx]);
            }
        }

        Ok(active_edges)
    }

    fn find_invalid_subtours(&self, k: usize, active_edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let num_nodes = self.instance.nodes.len();
        let start_node = self.instance.vehicles[k].start_node_id;
        let end_node = self.instance.vehicles[k].end_node_id;

        let mut graph: Vec<Vec<usize>> = vec![vec![]; num_nodes];
        for &(i, j) in active_edges {
            graph[i].push(j);
            graph[j].push(i);
        }

        let mut visited = vec![false; num_nodes];
        let mut bad_tours = Vec::new();

        for node in 0..num_nodes {
            if !visited[node] && !graph[node].is_empty() {
                let mut component = Vec::new();
                let mut stack = vec![node];

                while let Some(current) = stack.pop() {
                    if !visited[current] {
                        visited[current] = true;
                        component.push(current);

                        for &neighbor in &graph[current] {
                            stack.push(neighbor);
                        }
                    }
                }

                if !component.contains(&start_node) && !component.contains(&end_node) {
                    bad_tours.push(component);
                }
            }
        }

        bad_tours
    }

    fn apply_cut_set_constraints(
        &self,
        k: usize,
        bad_tours: Vec<Vec<usize>>,
        ctx: &mut MIPSolCtx,
    ) -> Result<(), anyhow::Error> {
        let num_nodes = self.instance.nodes.len();

        for tour in bad_tours {
            let subset_barr: Vec<usize> = (0..num_nodes).filter(|n| !tour.contains(n)).collect();

            let vars = self.variables;

            let cut_expr: grb::Expr = tour
                .iter()
                .flat_map(|&i| subset_barr.iter().map(move |&j| vars.x[k][i][j]))
                .sum();

            let y_trigger = self.variables.y[k][tour[0]];

            ctx.add_lazy(c!(cut_expr >= y_trigger))?;
        }

        Ok(())
    }
}

impl<'a> Callback for SubtourCallback<'a> {
    fn callback(&mut self, w: Where) -> Result<(), anyhow::Error> {
        let mut ctx = match w {
            Where::MIPSol(ctx) => ctx,
            _ => return Ok(()),
        };

        for k in 0..self.instance.vehicles.len() {
            let active_edges = self.get_active_edges(k, &ctx)?;

            if active_edges.is_empty() {
                continue;
            }

            let bad_tours = self.find_invalid_subtours(k, &active_edges);

            if bad_tours.is_empty() {
                continue;
            }

            self.apply_cut_set_constraints(k, bad_tours, &mut ctx)?;
        }

        Ok(())
    }
}
