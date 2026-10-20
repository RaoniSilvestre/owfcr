//! Flow-based algorithms: SSP (Successive Shortest Path) and NCC (Negative Cycle Canceling).
//!
//! Both operate on a directed flow representation (`Vec<i8>` indexed by `edge_to_idx`)
//! and share the residual-cost, solution-building and penalty helpers defined here.
//! The SSP variants also share turbine ordering, path search and flow augmentation.

mod ncc;
mod ssp;

use crate::PENALTY;
use crate::algorithms::mecanism;
use crate::types::{Idx, Mask, Problem, Solution};
use crate::utils;
pub use ncc::NegativeCycleCanceling;
pub use ssp::{SSPSortStrategy, Ssp};

/// Marginal cost of sending `delta` extra unit(s) of flow along the directed edge `u -> v`,
/// given the current `active_flows` and the crossing `active_mask`.
fn calculate_edge_cost(
    problem: &Problem,
    u: Idx,
    v: Idx,
    active_flows: &[i8],
    active_mask: &Mask,
    n: usize,
    delta: i8,
) -> f64 {
    let Problem {
        colisions,
        graph,
        cables,
        ..
    } = problem;

    let distance = graph[u][v];

    let dir_idx = utils::edge_to_idx(u, v, n);
    let inv_idx = utils::edge_to_idx(v, u, n);

    let net_flow = active_flows[dir_idx] - active_flows[inv_idx];

    // Inverse flow
    if net_flow < 0 {
        let inv_flow = net_flow.abs();
        let cost_now = cables.get_cable_cost(inv_flow, distance);

        // Delta will not change the direction of the flow
        if delta <= inv_flow {
            let cost_next = cables.get_cable_cost(inv_flow - delta, distance);
            cost_next - cost_now
            // Delta will change the direction of the flow
        } else {
            let forward_flow = delta - inv_flow;
            let cost_next = cables.get_cable_cost(forward_flow, distance);

            // We have a new u -> v cable so we need to verify colisions
            let crossings = colisions.count_crossings(u, v, active_mask);
            let crossing_penalty = f64::from(crossings) * PENALTY;

            (cost_next - cost_now) + crossing_penalty
        }
    // Direct flow
    } else {
        // Current cost
        let cost_now = if net_flow > 0 {
            cables.get_cable_cost(net_flow, distance)
        } else {
            0.0
        };

        //  Cost adding  `delta` units
        let cost_next = cables.get_cable_cost(net_flow + delta, distance);

        // If current netflow is zero, we need to add the possible crossing penalty
        let crossings = if net_flow == 0 {
            colisions.count_crossings(u, v, active_mask)
        } else {
            0
        };

        let crossing_penalty = f64::from(crossings) * PENALTY;

        (cost_next - cost_now) + crossing_penalty
    }
}

fn build_solution(problem: &Problem, active_flows: &[i8]) -> Solution {
    let n = problem.graph.node_count();

    let mut parents = vec![problem.graph.substation(); n];
    // review this.
    for (i, _) in active_flows
        .iter()
        .enumerate()
        .filter(|(_, f)| f.is_positive())
    {
        let (u, v) = utils::idx_to_edge(i, n);
        parents[u] = v;
    }
    let mut solution = Solution::from_parents(parents, problem);
    mecanism::evaluate(problem, &mut solution);
    solution
}
