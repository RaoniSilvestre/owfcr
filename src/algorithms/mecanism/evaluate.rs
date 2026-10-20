use crate::{
    PENALTY,
    types::{Problem, Solution},
};

pub fn evaluate(problem: &Problem, solution: &mut Solution) {
    let mut total_cost = 0.0;

    total_cost += evaluate_edges_cost(problem, solution);
    total_cost += evaluate_substation_limit(problem, solution);
    total_cost += evaluate_crossings(solution);

    solution.set_cost(total_cost);
}

fn evaluate_edges_cost(problem: &Problem, solution: &Solution) -> f64 {
    let substation = problem.graph.substation();
    let mut total = 0.0;
    for x in problem.graph.nodes() {
        if x == substation {
            continue;
        }
        let parent = solution.parents[x];
        let distance = problem.graph[x][parent];
        let flow = solution.flows[x];
        let cable = problem.cables.get_cable_cost(flow, distance);
        total += cable;
    }
    total
}

fn evaluate_substation_limit(problem: &Problem, solution: &Solution) -> f64 {
    let substation_idx = problem.graph.substation();
    let substation_limit = -problem.graph.node(substation_idx).w;
    let limit = i32::from(substation_limit);
    let excess = (solution.substation_connections - limit).max(0);
    f64::from(excess) * PENALTY
}

fn evaluate_crossings(solution: &Solution) -> f64 {
    f64::from(solution.crossings) * crate::PENALTY
}
