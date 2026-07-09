#![allow(clippy::useless_conversion)]

use super::cache::CallbackCache;
use crate::common::constants::{FRAC_TOLERANCE, INT_TOLERANCE};
use crate::solvers::exact::gurobi::ilp::DecisionVariables;
use grb::callback::{MIPNodeCtx, MIPSolCtx};
use grb::prelude::*;

type FractionalEdge = (usize, usize, f64);
type FractionalGraph = (Vec<FractionalEdge>, Vec<f64>);

pub fn get_fractional_graph(
    k: usize,
    ctx: &MIPNodeCtx,
    cache: &CallbackCache,
) -> Result<FractionalGraph, anyhow::Error> {
    let y_vals = ctx.get_solution(&cache.y_vars[k])?;
    let x_vals = ctx.get_solution(&cache.x_vars[k])?;

    let mut edges = Vec::with_capacity(x_vals.len());
    for (idx, &val) in x_vals.iter().enumerate() {
        if val > FRAC_TOLERANCE {
            let (i, j) = cache.x_edges[k][idx];
            edges.push((i, j, val));
        }
    }

    Ok((edges, y_vals))
}

pub fn get_active_edges(
    k: usize,
    ctx: &MIPSolCtx,
    cache: &CallbackCache,
) -> Result<Vec<(usize, usize)>, anyhow::Error> {
    let solution_vals = ctx.get_solution(&cache.x_vars[k])?;

    let num_nodes = cache.y_vars[k].len();
    let mut active_edges = Vec::with_capacity(num_nodes);

    for (idx, &val) in solution_vals.iter().enumerate() {
        if val > INT_TOLERANCE {
            active_edges.push(cache.x_edges[k][idx]);
        }
    }

    Ok(active_edges)
}

pub fn apply_fractional_cuts(
    k: usize,
    bad_tours: Vec<(Vec<usize>, usize)>,
    ctx: &mut MIPNodeCtx,
    variables: &DecisionVariables,
    num_nodes: usize,
) -> Result<(), anyhow::Error> {
    for (tour, src) in bad_tours {
        let subset_barr: Vec<usize> = (0..num_nodes).filter(|n| !tour.contains(n)).collect();

        let cut_expr: grb::Expr = subset_barr
            .iter()
            .flat_map(|&u| tour.iter().map(move |&v| variables.x[k][u][v]))
            .sum();

        let y_trigger = variables.y[k][src];
        ctx.add_cut(c!(cut_expr >= y_trigger))?;
    }

    Ok(())
}

pub fn apply_cut_set_constraints(
    k: usize,
    bad_tours: Vec<Vec<usize>>,
    ctx: &mut MIPSolCtx,
    variables: &DecisionVariables,
    num_nodes: usize,
) -> Result<(), anyhow::Error> {
    for tour in bad_tours {
        let dfj_expr: grb::Expr = tour.iter()
            .flat_map(|&i| tour.iter().filter(move |&&j| i != j)
            .map(move |&j| variables.x[k][i][j]))
            .sum();

        let max_edges = (tour.len() - 1) as f64;
        ctx.add_lazy(c!(dfj_expr <= max_edges))?;

        let subset_barr: Vec<usize> = (0..num_nodes).filter(|n| !tour.contains(n)).collect();
        let cut_expr: grb::Expr = subset_barr
            .iter()
            .flat_map(|&u| tour.iter().map(move |&v| variables.x[k][u][v]))
            .sum();

        for &v in &tour {
            let y_trigger = variables.y[k][v];
            ctx.add_lazy(c!(cut_expr.clone() >= y_trigger))?;
        }
    }
    Ok(())
}