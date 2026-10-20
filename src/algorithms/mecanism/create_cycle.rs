use crate::types::Idx;

/// We assume that the solution is a tree.
///
/// If we connect a parent to a child, it will disconnect the tree and make a infinite loop. So this
/// cycle verification is needed.
pub fn creates_cycle(parents: &[Idx], child: Idx, parent_candidate: Idx, substation: Idx) -> bool {
    let mut parent_candidate = parent_candidate;

    if parent_candidate == substation {
        return false;
    }

    while parent_candidate != substation {
        if parent_candidate == child {
            return true;
        }
        parent_candidate = parents[parent_candidate];
    }

    false
}
