//! Problem, Metadata of the problem and a trial defition module.

use crate::types::{Cables, FastIntersectionTester, Graph, Solution};
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct Problem {
    pub instance_name: String,
    pub colisions: Arc<FastIntersectionTester>,
    pub graph: Graph,
    pub cables: Cables,
}

pub struct Metadata {
    pub duration: Duration,
    pub algorithm: String,
    pub instance: String,
    pub instance_size: i32,
    pub seed: Option<i32>,
}

pub struct Trial {
    pub solution: Solution,
    pub metadata: Metadata,
}
