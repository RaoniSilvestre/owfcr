use crate::{
    algorithms::mecanism::creates_cycle,
    types::{Idx, Solution},
};

#[derive(Clone, Copy)]
pub struct Validator<'a> {
    pub solution: &'a Solution,
    pub max_flow: i8,
    pub child: Idx,
    pub parent_candidate: Idx,
}

impl Validator<'_> {
    /// Validates if
    //
    /// 1. i != k
    /// 2. i != substation
    /// 3. Creates cycles (not allowed in this VNS)
    /// 4. Creates invalid flow (any edge with flow greater than the maximum allowed by the most expensive cable)
    /// 5. Creates any crossings with the solution edges
    pub(super) fn is_valid_move(&self) -> bool {
        if self.child == self.parent_candidate || self.child == self.solution.substation {
            return false;
        }

        if creates_cycle(
            &self.solution.parents,
            self.child,
            self.parent_candidate,
            self.solution.substation,
        ) {
            return false;
        }

        if !self.is_valid_flow() {
            return false;
        }

        !self
            .solution
            .any_crossings(self.child, self.parent_candidate)
    }

    pub(super) fn weak_valid_move(&self) -> bool {
        if self.child == self.parent_candidate || self.child == self.solution.substation {
            return false;
        }

        if creates_cycle(
            &self.solution.parents,
            self.child,
            self.parent_candidate,
            self.solution.substation,
        ) {
            return false;
        }

        true
    }

    fn is_valid_flow(&self) -> bool {
        let subtree_i = self.solution.flows[self.child];
        let mut k = self.parent_candidate;
        while k != self.solution.substation {
            if self.solution.flows[k] + subtree_i > self.max_flow {
                return false;
            }
            k = self.solution.parents[k];
        }

        true
    }
}
