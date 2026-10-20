//! WFCR library
//!
//! The library has 4 algorithm implementation:
//! * Sweeping
//! * Sweeping multistart
//! * Simple Variable Neighbourhood Search (VNS)
//! * SSP with dikstra and normal weigths
//!
//! The first three were based on the paper from David Cazzaro et al. In there, he talks about the
//! algorithm and how they combined it with VNS.
//!
//! All of them implements the same interface, making it possible to gather data for every execution
//!
//! The library also contains structure to help to execute the algorithms N times for every test instance.
//!
//! All algorithms generate solution based on the basic formulation of WFCR with the following constraints:
//!
//! * Edge crossing is not allowed;
//! * The substation has a maximum capacity of edges;
//! * Every cable type has a maximum capacity and price associated.

// Using explicit simd to optimize some things.
#![feature(portable_simd)]

pub mod algorithms;
pub mod cli;
mod codec;
mod log;
mod runner;
pub mod types;
mod utils;

use tracing_subscriber::EnvFilter;

/// Starts the logging system
///
/// It defines the logging style and routes every info!() call to the stdout.
pub fn start_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .with_line_number(false)
        .with_file(false)
        .compact()
        .init();
}

/// Value added to any edge in any solution that break a constraint of the problem.
pub const PENALTY: f64 = 1_000_000_000.0;
