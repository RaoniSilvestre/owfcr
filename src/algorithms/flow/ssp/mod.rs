//! Sucessive Shortest Path with Dijkstra using Normal Weights (SSP / DIJNOR).
//!
//! This algorithm works by getting the shortest path to the substation from each turbine. But for
//! each of the turbines, it can take previous edges already used. This is both the strength and the
//! weakness of this algorithm.
//!
//! The strength is because it will seek the local optimum path if you only pass to each vertex one
//! time. The weakness is because it will tend to form long strings of edges, which will top the
//! capacity of the cable closer to the substation.
//!
//! This implementation is based on the marc jenne paper about the topic. In the paper, it compares
//! four different strategies for this SSP heuristic. The best one being DIJNOR. Also in the paper it
//! also tries to model the WFCR in MCF context, but finds a lot of friction in implementing any
//! other flow-based classic algorithm.

mod algorithm;
mod sort;

pub use sort::SSPSortStrategy;

/// Sucessive shortest path using dikstra with normal weights
#[derive(Clone)]
pub struct Ssp {
    /// Name of the algorithm, changes acording to the `sort_strategy`.
    pub name: String,
    /// Sorting strategy of the turbines.
    pub sort_strategy: SSPSortStrategy,
}

impl Ssp {
    /// Initializes the SSP struct
    #[must_use]
    pub fn new(sort_strategy: SSPSortStrategy) -> Self {
        Self {
            name: sort_strategy.to_string(),
            sort_strategy,
        }
    }
}
