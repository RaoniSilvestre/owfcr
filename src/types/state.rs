use crate::types::Idx;
use std::cmp::Ordering;

/// Heap state used in dikstra's algorithm (SSP)
#[derive(Copy, Clone, PartialEq)]
pub struct State {
    /// Cost to reach this node
    pub cost: f64,
    /// Associated node
    pub node: Idx,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
