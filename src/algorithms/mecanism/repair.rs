use crate::{
    algorithms::mecanism,
    types::{Idx, Problem, Solution},
};

pub fn repair(problem: &Problem, solution: &mut Solution) {
    let graph = &problem.graph;
    let substation = graph.substation();

    let disconnected_nodes: Vec<Idx> = graph
        .nodes()
        .filter(|&node| {
            node != substation && !reaches_substation(&solution.parents, node, substation)
        })
        .collect();

    if disconnected_nodes.is_empty() {
        return;
    }

    for &u in &disconnected_nodes {
        if reaches_substation(&solution.parents, u, substation) {
            continue;
        }

        let mut best_target: Option<Idx> = None;
        let mut best_distance = f64::MAX;

        for v in graph.nodes() {
            if u == v || !reaches_substation(&solution.parents, v, substation) {
                continue;
            }

            let validator = mecanism::Validator {
                solution,
                max_flow: problem.cables.best_cable_conn(),
                child: u,
                parent_candidate: v,
            };

            if validator.is_valid_move() {
                let distance = problem.graph[u][v];
                if distance < best_distance {
                    best_distance = distance;
                    best_target = Some(v);
                }
            }
        }

        if let Some(target) = best_target {
            solution.swap_edge(u, target);
        } else {
            solution.swap_edge(u, substation);
        }
    }
}

fn reaches_substation(parents: &[Idx], mut node: Idx, substation: Idx) -> bool {
    let mut visited_count = 0;
    let max_nodes = parents.len();

    while node != substation {
        node = parents[node];
        visited_count += 1;

        if visited_count > max_nodes {
            return false;
        }
    }

    true
}
