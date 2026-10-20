use crate::{
    algorithms::mecanism,
    types::{Idx, Problem, Solution},
};
use rand::{RngExt, rngs::StdRng, seq::IndexedRandom};
use tracing::instrument;

#[instrument(skip_all, name = "Shaking")]
pub fn shake(problem: &Problem, solution: &mut Solution, rng: &mut StdRng, k: i32) {
    let Problem { cables, graph, .. } = problem;

    let node_count = graph.node_count();
    let substation = graph.substation();

    let max_flow = cables.best_cable_conn();

    for _ in 0..k {
        // Try to find a valid move, stop at the first successfull move found.
        loop {
            let i = rng.random_range(0..node_count);

            if i == substation {
                continue;
            }

            let valid_targets: Vec<Idx> = graph
                .nodes()
                .filter(|&k| {
                    let validator = mecanism::Validator {
                        solution,
                        max_flow,
                        child: i,
                        parent_candidate: k,
                    };

                    validator.weak_valid_move()
                })
                .collect();

            // If we found any, choose one and apply
            if let Some(&target_k) = valid_targets.choose(rng) {
                solution.swap_edge(i, target_k);
                break; // Next 'k'
            }
        }
    }

    mecanism::evaluate(problem, solution);
}
