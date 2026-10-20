use super::load_algorithms;
use crate::algorithms::Algorithm;
use crate::cli::RunAllArgs;
use crate::types::Problem;
use crate::{
    codec::{self, Repo},
    runner::Runner,
};
use color_eyre::eyre::Result;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::fs::DirEntry;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::thread;
use std::time::Duration;
use std::{
    fs::{self},
    path::Path,
    time::Instant,
};
use tracing::{info, instrument};

#[instrument(skip_all, err)]
pub fn run_all(run_all: &RunAllArgs) -> Result<()> {
    let path = Path::new(&run_all.instances);

    let directories: Vec<fs::DirEntry> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "turb"))
        .collect();

    info!("Iniciando execução dos algoritmos");

    let start = Instant::now();

    let executions = Arc::new(AtomicI64::new(0));

    let runner = Runner {
        repo: Repo::new("sqlite.db", executions.clone()),
    };

    let problems: Vec<Box<Problem>> = directories
        .par_iter()
        .map(DirEntry::path)
        .map(|p| codec::load(&p).unwrap())
        .map(Box::new)
        .collect();

    let mut tasks: Vec<(Box<Problem>, Box<dyn Algorithm>)> = Vec::new();

    let algorithms = load_algorithms(Path::new(&run_all.config)).unwrap();

    for problem in &problems {
        for algorithm in &algorithms {
            for _ in 0..run_all.reps {
                tasks.push((problem.clone(), algorithm.clone_box()));
            }
        }
    }

    let total_tasks = tasks.len() as i64;

    info!("Tasks criadas.");

    tasks.into_par_iter().for_each(|(problem, mut algorithm)| {
        runner.run(algorithm.as_mut(), &problem);
    });

    info!("Tasks finalizadas.");

    let elapsed = start.elapsed();

    info!("Execução dos algoritmos finalizada em {:?}", elapsed);

    while executions.load(Ordering::Relaxed) < total_tasks {
        thread::sleep(Duration::from_secs(5));
    }

    Ok(())
}
