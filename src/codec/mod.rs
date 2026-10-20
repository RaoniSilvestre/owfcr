//! Codification/Decodification module
//!
//! Module responsible to save trials in the database and load from disk the test instances.

mod load;
mod repo;

pub use load::load;
pub use repo::Repo;
