use crate::{
    algorithms::{Algorithm, mecanism},
    log,
    types::{Problem, Solution},
};
use rand::{SeedableRng, rngs::StdRng};
use std::time::Instant;
use tracing::{info, instrument};

/// Variable Neighbourhood Search (VNS)
///
/// This algorithm works by iteratively trying to build a better solution over time, doing small
/// changes to the incumbent solution.
///
/// First, another algorithm runs to generate a viable first solution. This solution must be a tree,
/// since the VNS consider that to be true to improve the solution.
///
/// After that, the algorithm starts to iteratively do:
///
/// Let y be the current solution
///
/// 1. Shaking: Gets a new solution y' by changing `k` edges on y, even if that move
///    doesn't improve the solution.
/// 2. Local search: On y', it tries to find the local optimum (changing y'). If the local
///    optimum  is better than y, then y = y'.
/// 3. Repeat until the iteration limit is reached.
///
/// Most of the time of this algorithm is the local search step, if it is improved, the algorithm
/// would be able to run more times and get better results.
pub struct VariableNeighborhoodSearch {
    /// Name of the algorithm.
    pub name: String,
    /// Inner algorithm that generates an initial solution.
    pub algorithm: Box<dyn Algorithm>,
    /// Iteration limit for the execution of the VNS.
    pub max_iterations: usize,
    /// Some operations are random (mainly in `shake` phase), so the seed is configurable.
    pub seed: u64,
    /// Max amount of shaking that it can do.
    pub k_max: i32,
    /// How many iterations the algorithm went.
    iters: usize,
}

impl Clone for VariableNeighborhoodSearch {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            algorithm: self.algorithm.clone_box(),
            max_iterations: self.max_iterations,
            seed: self.seed,
            k_max: self.k_max,
            iters: self.iters,
        }
    }
}

impl Algorithm for VariableNeighborhoodSearch {
    fn name(&self) -> String {
        format!("{} ({} iters)", self.name, self.max_iterations)
    }

    fn clone_box(&self) -> Box<dyn Algorithm> {
        Box::new(self.clone())
    }

    #[instrument(skip_all, name = "VNS")]
    fn execute(&mut self, problem: &Problem) -> Solution {
        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut current = self.algorithm.execute(problem);

        assert!(mecanism::is_tree(&current, problem));

        let initial = current.clone();

        info!("Initial solution: {}", current.cost);

        let mut k = 1;

        let mut last_log_time = Instant::now();

        while self.iters < self.max_iterations {
            let mut candidate = current.clone();

            mecanism::shake(problem, &mut candidate, &mut rng, k);
            self.iters += mecanism::local_search(problem, &mut candidate);

            increase_shaking_force(&mut current, &mut k, candidate, self.k_max);

            log::log_progress(
                self.iters,
                self.max_iterations,
                &mut last_log_time,
                current.cost,
                initial.cost,
            );
        }

        mecanism::evaluate(problem, &mut current);

        log::log_result(self.iters, self.max_iterations, current.cost, initial.cost);

        current
    }
}

impl VariableNeighborhoodSearch {
    #[must_use]
    pub fn new(
        name: String,
        max_iterations: usize,
        seed: u64,
        algorithm: Box<dyn Algorithm>,
        k_max: i32,
    ) -> Self {
        Self {
            name,
            max_iterations,
            seed,
            algorithm,
            k_max,
            iters: 0,
        }
    }
}

fn increase_shaking_force(current: &mut Solution, k: &mut i32, local: Solution, k_max: i32) {
    if local.cost < current.cost {
        *current = local;
        *k = 1;
    } else {
        *k += 1; // Didn't get better, adds more shakes.
        if *k > k_max {
            *k = 1;
        }
    }
}
