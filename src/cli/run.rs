use crate::cli::{RunArgs, load_algorithms};
use crate::{
    codec::{self, Repo},
    runner::Runner,
};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::thread;
use std::time::Duration;
use tracing::info;

pub fn run(run: &RunArgs) {
    info!("Initializing repository in {}", run.db);

    let mut algorithms = load_algorithms(Path::new(&run.config)).unwrap();
    let problem = codec::load(&run.file).expect("Failed to load instance");

    let executions = Arc::new(AtomicI64::new(0));

    let runner = Runner {
        repo: Repo::new(&run.db, executions.clone()),
    };

    let total_tasks = algorithms.len() as i64;

    algorithms.par_iter_mut().for_each(|algorithm| {
        info!("Starting algorithm execution.");
        runner.run(algorithm.as_mut(), &problem);
    });

    while executions.load(Ordering::Relaxed) < total_tasks {
        thread::sleep(Duration::from_millis(50));
    }
}
