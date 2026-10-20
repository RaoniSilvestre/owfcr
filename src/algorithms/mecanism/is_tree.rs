use crate::types::{Problem, Solution};

#[must_use]
pub fn is_tree(solution: &Solution, problem: &Problem) -> bool {
    let node_count = problem.graph.node_count();
    let substation = problem.graph.substation();

    // Must not be a edge going out of substation
    if solution.parents[substation] != problem.graph.substation() {
        return false;
    }

    // Every node must be able to reach the substation with at most `node_count` iterations. (Might
    // be in a loop otherwise).
    for start in 0..node_count {
        if start == substation {
            continue;
        }
        let mut current = start;
        for _ in 0..node_count {
            if current == substation {
                break;
            }
            current = solution.parents[current];
        }
        if current != substation {
            return false;
        }
    }
    true
}
