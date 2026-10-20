use crate::{
    algorithms::{mecanism, transgenetic::vetor::TransgeneticVector},
    types::{Problem, Solution},
};
use rand::{RngExt, rngs::StdRng};

/// Transposon clássico: perturbação (shake) + busca local geral.
pub struct Transposon;

impl TransgeneticVector for Transposon {
    fn transcribe(
        &self,
        problem: &Problem,
        receptor: &Solution,
        rng: &mut StdRng,
        iters: &mut usize,
    ) -> Solution {
        let mut receptor = receptor.clone();
        let k = rng.random_range(2..=5);
        mecanism::shake(problem, &mut receptor, rng, k);
        *iters += mecanism::local_search(problem, &mut receptor);

        receptor
    }
}
