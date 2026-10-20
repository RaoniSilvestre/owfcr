//! Types used in the library.
//!
//! All major types that are shared in some way are in this modulo tree.

mod cables;
mod graph;
mod intersection;
mod node;
mod problem;
mod solution;
mod state;

pub use cables::*;
pub use graph::*;
pub use intersection::*;
pub use node::*;
pub use problem::*;
pub use solution::*;
pub use state::*;

/// The default index size used in the library. Just shorter name of the petgraph default.
pub type Idx = usize;
/// Usually is defined when you have some ordenation with the relative angle of the turbines and the
/// substation.
pub type Angle = f64;
/// The cost of any given edge, cable or edge using a given cable type.
pub type Cost = f64;
