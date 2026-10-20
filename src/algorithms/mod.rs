mod flow;
mod genetic;
mod mecanism;
mod memetic;
mod random;
mod sweep;
mod transgenetic;
mod vns;

use crate::types::{Problem, Solution};
pub use flow::{NegativeCycleCanceling, SSPSortStrategy, Ssp};
pub use genetic::Genetic;
pub use memetic::Memetic;
pub use random::RandomAlgorithm;
pub use sweep::Sweeping;
pub use sweep::SweepingMultiStart;
pub use transgenetic::Transgenetic;
pub use vns::VariableNeighborhoodSearch;

pub trait Algorithm: Send + Sync {
    fn name(&self) -> String;
    fn execute(&mut self, problem: &Problem) -> Solution;
    fn clone_box(&self) -> Box<dyn Algorithm>;
}
