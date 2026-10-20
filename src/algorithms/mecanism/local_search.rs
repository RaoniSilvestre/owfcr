use crate::{
    algorithms::mecanism::{self, validate_move::Validator},
    types::{Idx, Problem, Solution},
};
use tracing::{instrument, warn};

/// The goal here is to find the best edge movement to the solution.
///
/// While improvement is made, we go to every node pair (i, k) and try to find a better solution. If
/// one is found, we update the current solution and try again until no better solution is found.
#[instrument(skip_all, name = "LocalSearch")]
#[must_use]
pub fn local_search(problem: &Problem, solution: &mut Solution) -> usize {
    let Problem { cables, graph, .. } = problem;

    let substation = graph.substation();
    let substation_limit = graph.substation_limit();

    let max_flow = cables.best_cable_conn();

    let mut improved = true;

    let mut count_iters = 0;

    while improved {
        improved = false;
        let mut best_delta = f64::INFINITY;
        let mut best_move: Option<(Idx, Idx)> = None;

        for child in graph.nodes() {
            if child == substation {
                continue;
            }

            let parent = solution.parents[child];
            let child_flow = solution.flows[child];

            for candidate in graph.nodes() {
                let validator = Validator {
                    solution,
                    max_flow,
                    child,
                    parent_candidate: candidate,
                };

                if !validator.is_valid_move() {
                    continue;
                }

                count_iters += 1;

                let delta = move_delta(
                    problem,
                    solution,
                    child,
                    parent,
                    candidate,
                    child_flow,
                    substation_limit,
                );

                if delta < best_delta && delta < -0.1 {
                    best_delta = delta;
                    best_move = Some((child, candidate));
                }
            }
        }

        // If found a improvement, apply it definitively
        if let Some((child, candidate)) = best_move
            && best_delta < 0.
        {
            solution.swap_edge(child, candidate);
            improved = true;
        }

        // Evaluate the new cost
        mecanism::evaluate(problem, solution);

        count_iters += 1;
    }

    count_iters
}

#[allow(clippy::too_many_arguments)]
fn move_delta(
    problem: &Problem,
    solution: &mut Solution,
    child_node: Idx,
    parent_node: Idx,
    target_node: Idx,
    child_flow: i8,
    substation_connection_limit: i8,
) -> f64 {
    let Problem {
        colisions,
        graph,
        cables,
        ..
    } = problem;

    let parents = &solution.parents;
    let substation_connections = solution.substation_connections;
    let substation = graph.substation();

    let old_distance = graph[child_node][parent_node];
    let new_distance = graph[child_node][target_node];

    let old_cost = cables.get_cable_cost(child_flow, old_distance);
    let new_cost = cables.get_cable_cost(child_flow, new_distance);

    let mut delta = new_cost - old_cost;

    let mut x = parent_node;

    while x != substation {
        let parent = parents[x];
        let distance = graph[x][parent];
        delta -= cables.get_cable_cost(solution.flows[x], distance);
        solution.flows[x] -= child_flow;
        delta += cables.get_cable_cost(solution.flows[x], distance);
        x = parents[x];
    }

    x = target_node;
    while x != substation {
        let parent = parents[x];
        let distance = graph[x][parent];
        delta -= cables.get_cable_cost(solution.flows[x], distance);
        solution.flows[x] += child_flow;
        delta += cables.get_cable_cost(solution.flows[x], distance);
        x = parents[x];
    }

    delta += delta_substation(
        parent_node,
        target_node,
        substation,
        substation_connections as i8,
        substation_connection_limit,
    );

    delta -= f64::from(colisions.count_crossings(child_node, parent_node, &solution.mask))
        * crate::PENALTY;

    delta += f64::from(colisions.count_crossings(child_node, target_node, &solution.mask))
        * crate::PENALTY;

    let mut x = target_node;
    while x != substation {
        solution.flows[x] -= child_flow;
        x = parents[x];
    }
    x = parent_node;
    while x != substation {
        solution.flows[x] += child_flow;
        x = parents[x];
    }

    delta
}

fn delta_substation(j: Idx, k: Idx, substation: Idx, sub_conns: i8, limit: i8) -> f64 {
    let new_conns = sub_conns - i8::from(j == substation) + i8::from(k == substation);

    let old_penalty = f64::from((sub_conns - limit).max(0)) * crate::PENALTY;
    let new_penalty = f64::from((new_conns - limit).max(0)) * crate::PENALTY;

    new_penalty - old_penalty
}
