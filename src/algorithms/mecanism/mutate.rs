use crate::{
    algorithms::mecanism,
    types::{Problem, Solution},
};
use rand::rngs::StdRng;

pub fn mutate(problem: &Problem, solution: &mut Solution, rng: &mut StdRng, mutation_rate: f64) {
    let k = (mutation_rate * problem.graph.node_count() as f64).round() as i32;
    if k > 0 {
        mecanism::shake(problem, solution, rng, k);
    }
}
