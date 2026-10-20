//! Application entrypoint.
//!
//! Initializes error handling, logging and passes the execution to the CLI.

use clap::Parser;
use color_eyre::eyre::Result;
use wfcr::cli::Cli;

/// Entrypoint
fn main() -> Result<()> {
    color_eyre::install()?;
    wfcr::start_logger();

    let cli = Cli::parse();
    cli.run()
}
