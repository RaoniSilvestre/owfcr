use crate::{
    algorithms::{Algorithm, mecanism},
    log,
    types::{Problem, Solution},
};
use rand::{SeedableRng, rngs::StdRng};
use std::time::Instant;
use tracing::instrument;

pub struct Memetic {
    pub name: String,
    pub algorithm: Vec<Box<dyn Algorithm>>,
    pub max_iterations: usize,
    pub seed: u64,
    pub k_max: i32,
    pub iters: usize,
    pub population_size: usize,
}

impl Clone for Memetic {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            algorithm: self.algorithm.iter().map(|a| a.clone_box()).collect(),
            max_iterations: self.max_iterations,
            seed: self.seed,
            k_max: self.k_max,
            iters: self.iters,
            population_size: self.population_size,
        }
    }
}

impl Algorithm for Memetic {
    fn name(&self) -> String {
        format!("{} ({} iters)", self.name, self.max_iterations)
    }

    fn clone_box(&self) -> Box<dyn Algorithm> {
        Box::new(self.clone())
    }

    #[instrument(skip_all, name = "Memetic")]
    fn execute(&mut self, problem: &Problem) -> Solution {
        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut population = mecanism::populate(
            &mut self.algorithm,
            self.population_size,
            30,
            problem,
            &mut rng,
        );

        let size = population.len();

        let best_initial = population[0].clone();

        let mut last_log_time = Instant::now();

        while self.iters < self.max_iterations {
            let (p1, p2) = mecanism::tournament(&population, &mut rng, 2);

            let mut offspring = mecanism::crossover(problem, p1, p2, &mut rng);

            self.iters += mecanism::local_search(problem, &mut offspring);

            let is_duplicate = population.iter().any(|p| p.mask == offspring.mask);
            let worst_cost = population.last().unwrap().cost;

            if !is_duplicate && offspring.cost < worst_cost {
                population[size - 1] = offspring;
                population.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
            }

            log::log_progress(
                self.iters,
                self.max_iterations,
                &mut last_log_time,
                population[0].cost,
                best_initial.cost,
            );
        }

        log::log_result(
            self.iters,
            self.max_iterations,
            population[0].cost,
            best_initial.cost,
        );

        population.remove(0)
    }
}

impl Memetic {
    #[must_use]
    pub fn new(
        name: String,
        max_iterations: usize,
        seed: u64,
        algorithm: Vec<Box<dyn Algorithm>>,
        k_max: i32,
        population_size: usize,
    ) -> Self {
        Self {
            name,
            max_iterations,
            seed,
            algorithm,
            k_max,
            iters: 0,
            population_size,
        }
    }
}
