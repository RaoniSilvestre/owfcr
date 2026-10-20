use crate::{
    algorithms::mecanism::{self, creates_cycle},
    types::{Problem, Solution},
};
use rand::{Rng, seq::SliceRandom};

pub fn crossover(problem: &Problem, p1: &Solution, p2: &Solution, rng: &mut impl Rng) -> Solution {
    let mut parents = vec![problem.graph.substation(); problem.graph.node_count()];
    let mut divergent_nodes = Vec::new();

    for u in problem.graph.nodes() {
        let a = p1.parents[u];
        let b = p2.parents[u];

        if a == b {
            parents[u] = a;
        } else {
            divergent_nodes.push(u);
        }
    }

    divergent_nodes.shuffle(rng);

    for u in divergent_nodes {
        let a = p1.parents[u];
        let b = p2.parents[u];

        let (better, worse) = if p1.cost <= p2.cost { (a, b) } else { (b, a) };

        if !creates_cycle(&parents, u, better, problem.graph.substation()) {
            parents[u] = better;
        } else if !creates_cycle(&parents, u, worse, problem.graph.substation()) {
            parents[u] = worse;
        } else {
            parents[u] = problem.graph.substation();
        }
    }

    let mut child = Solution::from_parents(parents, problem);

    mecanism::repair(problem, &mut child);
    mecanism::evaluate(problem, &mut child);

    child
}
