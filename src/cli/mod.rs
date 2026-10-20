//! CLI module
//!
//! It exposes a CLI to interact with the algorithms, run them with one or all the instances, with
//! any hyper parameter.

mod run;
mod run_all;
mod spec;

use crate::algorithms::Algorithm;
use crate::cli::spec::AlgorithmSpec;
use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;
use std::path::PathBuf;
use std::{fs, path::Path};

/// CLI to run WFCR algorithms and save the results.
#[derive(Parser)]
#[command(name = "wfcr")]
#[command(about = "CLI to run WFCR algorithms and save the results.")]
pub struct Cli {
    /// Available commands to run the algorithms.
    ///
    /// * `run`: run one algorithm with one instance defined by the user
    /// * `run-all`: run all the algorithms defined in `wfcr.toml` with all the test instances.
    ///
    /// Also by default run them all 30 times, which is configurable.
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    /// Runs the command defined by the user
    ///
    /// # Errors
    ///
    /// If the inner command execution fail, the function fails.
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Run(args) => {
                run::run(&args);

                Ok(())
            }
            Commands::RunAll(args) => run_all::run_all(&args),
        }
    }
}

/// Possible commands to run the program.
#[derive(Subcommand)]
enum Commands {
    /// Runs a specific algorithm to one instance of WFCR
    Run(RunArgs),

    /// Run all the algorithms with all instances defined in `wfcr.toml`
    RunAll(RunAllArgs),
}

#[derive(Parser)]
struct RunArgs {
    /// Instance path (.turb)
    #[arg(short, long)]
    file: PathBuf,

    /// Sqlite DB path
    #[arg(long, default_value = "sqlite.db")]
    db: String,

    /// TOML file listing the algorithms to run
    #[arg(long, default_value = "wfcr.toml")]
    config: String,
}

#[derive(Parser)]
struct RunAllArgs {
    /// Directory with the instances
    #[arg(long, default_value = "./instances")]
    instances: String,

    /// Reps per instance
    #[arg(long, default_value_t = 30)]
    reps: usize,

    /// TOML file listing the algorithms to run
    #[arg(long, default_value = "wfcr.toml")]
    config: String,
}

#[derive(Deserialize)]
struct Config {
    algorithms: Vec<AlgorithmSpec>,
}

fn load_algorithms(path: &Path) -> Result<Vec<Box<dyn Algorithm>>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;

    config
        .algorithms
        .into_par_iter()
        .map(TryInto::try_into)
        .collect()
}
