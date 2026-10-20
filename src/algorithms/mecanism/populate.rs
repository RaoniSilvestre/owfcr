use crate::{
    algorithms::{Algorithm, mecanism},
    types::{Problem, Solution},
};
use rand::rngs::StdRng;
use std::cmp::Ordering;

pub fn populate(
    algorithms: &mut [Box<dyn Algorithm>],
    population_size: usize,
    shake_intensity: i32,
    problem: &Problem,
    rng: &mut StdRng,
) -> Vec<Solution> {
    let mut population: Vec<Solution> = Vec::with_capacity(population_size);

    let mut count = 0;
    while population.len() < population_size {
        let pos = count % algorithms.len();

        let mut solution = if count >= algorithms.len() {
            population[count - algorithms.len()].clone()
        } else {
            algorithms[pos].execute(problem)
        };

        mecanism::shake(problem, &mut solution, rng, shake_intensity);

        let _ = mecanism::local_search(problem, &mut solution);

        population.push(solution);

        count += 1;
    }

    population.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap_or(Ordering::Equal));

    population
}
