//! Runner module
//!
//! Just holds the Runner struct.

use crate::{
    algorithms::Algorithm,
    codec::Repo,
    types::{Metadata, Problem, Trial},
};
use std::time::Instant;
use tracing::{Span, error, field, info, instrument};

/// A runner struct
///
/// This holds a reference to the database and it is responsible to execute the algorithms for a
/// given problem instance and save the results in the database.
pub struct Runner {
    /// A little abstraction of the rusqlite lib to save Trials.
    pub repo: Repo,
}

impl Runner {
    /// Run a given algorithm in a given problem instance.
    ///
    /// Executes the algorithm in a given problem, getting metadata about the execution and saving
    /// it as a `Trial`.
    ///
    /// The trial is saved into the database
    #[instrument(
        skip_all, 
        name = "Runner",
        fields(
            algorithm = %algorithm.name(),
            size = problem.graph.node_count(),
            duration_ms = field::Empty,
            total_cost = field::Empty,
            instance = %problem.instance_name
        )
    )]
    pub fn run(&self, algorithm: &mut dyn Algorithm, problem: &Problem) {
        info!("Starting execution.");

        let start = Instant::now();
        let solution = algorithm.execute(problem);
        let elapsed = start.elapsed();

        let duration_ms = elapsed.as_millis() as u64;

        Span::current().record("ms", duration_ms);
        Span::current().record("cost", solution.cost);

        info!("Execution ended.");

        let metadata = Metadata {
            duration: elapsed,
            algorithm: algorithm.name(),
            instance: problem.instance_name.clone(),
            instance_size: problem.graph.node_count() as i32,
            seed: None,
        };

        let trial = Trial { solution, metadata };

        if let Err(e) = self.repo.save_trial(trial) {
            error!("Failed to save trial in database {e}");
        }
    }
}
