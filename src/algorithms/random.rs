use rand::{
    SeedableRng,
    rngs::StdRng,
    seq::{IndexedRandom, SliceRandom},
};

use crate::{
    algorithms::{Algorithm, mecanism},
    types::{Idx, Problem, Solution},
    utils,
};

/// Generates a completely random spanning tree rooted at the substation.
///
/// It randomly attaches every turbine to an already-connected node, which guarantees by
/// construction that the result is always a tree with the substation as the root.
#[derive(Clone)]
pub struct RandomAlgorithm {
    seed: u64,
    counter: u64,
}

impl RandomAlgorithm {
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self { seed, counter: 0 }
    }
}

impl Algorithm for RandomAlgorithm {
    fn name(&self) -> String {
        String::from("Random")
    }

    fn clone_box(&self) -> Box<dyn Algorithm> {
        Box::new(self.clone())
    }

    fn execute(&mut self, problem: &Problem) -> Solution {
        let graph = &problem.graph;
        let n = graph.node_count();
        let substation = graph.substation();

        let mut rng = StdRng::seed_from_u64(self.seed.wrapping_add(self.counter));
        self.counter += 1;

        let mut turbines: Vec<Idx> = graph
            .nodes()
            .into_iter()
            .filter(|&i| i != substation)
            .collect();
        turbines.shuffle(&mut rng);

        let mut pool: Vec<Idx> = vec![substation];
        let mut parents: Vec<Option<Idx>> = vec![None; n];

        let mut mask = vec![0u64; problem.colisions.blocks_per_row];

        for v in turbines {
            let valid: Vec<Idx> = pool
                .iter()
                .copied()
                .filter(|&u| problem.colisions.count_crossings(v, u, &mask) == 0)
                .collect();

            let parent = if valid.is_empty() {
                *pool.choose(&mut rng).unwrap()
            } else {
                *valid.choose(&mut rng).unwrap()
            };

            parents[v] = Some(parent);
            pool.push(v);

            let eid = utils::edge_id(v, parent);
            mask[eid / 64] |= 1 << (eid % 64);
        }

        let complete: Vec<Idx> = parents.iter().map(|p| p.unwrap_or(substation)).collect();

        let mut solution = Solution::from_parents(complete, problem);
        mecanism::evaluate(problem, &mut solution);

        solution
    }
}

