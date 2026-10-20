//! Negative Cycle Canceling (NCC).
//!
//! Wrapper algorithm: runs an inner algorithm to build an initial feasible flow, then
//! repeatedly cancels negative cycles in the residual graph to reduce the cost.
//!
//! The crossing mask is kept consistent with the flow on every cancellation, so the
//! residual costs always account for crossings correctly.
//!
//! Based on the algorithm described in [NNC article](https://arxiv.org/abs/1908.02129)

use super::build_solution;
use crate::algorithms::flow::calculate_edge_cost;
use crate::algorithms::{Algorithm, mecanism};
use crate::types::{Idx, Mask, Problem, Solution};
use crate::utils;

/// Negative Cycle Canceling: improves a flow produced by an inner algorithm by
/// canceling cost-reducing cycles.
pub struct NegativeCycleCanceling {
    pub name: String,
    pub algorithm: Box<dyn Algorithm>,
}

impl Clone for NegativeCycleCanceling {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            algorithm: self.algorithm.clone_box(),
        }
    }
}

impl Algorithm for NegativeCycleCanceling {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn clone_box(&self) -> Box<dyn Algorithm> {
        Box::new(self.clone())
    }

    /// Execute the NCC.
    ///
    /// Pseudocode:
    ///
    ///  Input: Graph G, costs c, edge lengths len
    ///  Result: A feasible flow f in G
    /// ```text
    /// f := InitializeFlow(G, len), ∆ := InitialDelta
    /// while ∆ != NULL do
    ///   (R, γ) := ComputeResidualGraph(G, c, f, ∆)
    ///     RunBellmanFord(R, γ)
    ///     found := false
    ///     foreach e ∈ E(R) do
    ///       W := FindNegativeClosedWalk(R, e)
    ///       C := DecomposeWalkIntoCycles(W )
    ///       foreach C ∈ C do
    ///         if |C| ≥ 3 and γ(C) < 0 then
    ///           f := AddFlowOnCycle(f, C, ∆)
    ///           found := true
    ///     if found then break
    ///   ∆ := NextDelta(∆, found)
    /// return f
    /// ```
    fn execute(&mut self, problem: &Problem) -> Solution {
        let (mut flow, mut mask) = self.initialize(problem);
        let initial_cost = solution_cost(problem, &flow);
        let mut current_cost = initial_cost;
        let node_count = problem.graph.node_count();

        let mut delta = Delta::new(problem.cables.best_cable_conn());

        loop {
            let current_delta = delta.current;
            let mut cancelled = false;

            // Compute residual graph and run bellman ford steps.
            let r = bellman_ford(problem, &mask, &flow, current_delta);

            let mut start = (0, 0);

            // Find negative closed walk
            'outer: while let Some((w, i, j)) =
                find_negative_closed_walk(problem, &r, &flow, &mask, current_delta, start)
            {
                start = (i, j);

                let cycles = decompose_walk_into_cycles(&w, node_count);

                for c in &cycles {
                    let mut scratch_flow = flow.clone();
                    let mut scratch_mask = mask.clone();

                    add_flow_on_cycle(
                        &mut scratch_flow,
                        &mut scratch_mask,
                        c,
                        node_count,
                        current_delta,
                    );

                    let scratch_solution = build_solution(problem, &scratch_flow);

                    if c.len() > 3
                        && scratch_solution.cost < current_cost
                        && mecanism::is_tree(&scratch_solution, problem)
                    {
                        add_flow_on_cycle(&mut flow, &mut mask, c, node_count, current_delta);
                        current_cost = scratch_solution.cost;
                        cancelled = true;
                        break 'outer;
                    }
                }
            }

            if !delta.next(cancelled) {
                break;
            }
        }

        assert!(initial_cost >= current_cost);

        build_solution(problem, &flow)
    }
}

impl NegativeCycleCanceling {
    #[must_use]
    pub fn new(name: String, algorithm: Box<dyn Algorithm>) -> Self {
        Self { name, algorithm }
    }

    fn initialize(&mut self, problem: &Problem) -> (Vec<i8>, Mask) {
        let graph = &problem.graph;
        let initial = self.algorithm.execute(problem);
        let n = problem.graph.node_count();

        let mut flows = vec![0i8; n * n];

        for x in graph.nodes() {
            if x == problem.graph.substation() {
                continue;
            }
            flows[utils::edge_to_idx(x, initial.parents[x], n)] = initial.flows[x];
        }
        let mask = initial.mask.clone();

        (flows, mask)
    }
}

/// A node from the bellmanford residual graph.
///
/// For this modified version, we need to store the "first best" and the "second best" path to any
/// `u` node. This happens because we need to avoid "U-turns": a cycle that is like v -(1)-> u -(-1)-> v.
/// Consider that this `u` node has a second best way: k -(2)-> u.
///
/// The logic is that we will hold both v -> u and k -> u. If the best way to `v` is from `u`, we
/// see that `v` is the best way to `u`, and choose `k` instead.
///
/// This is kinda odd to understand but plot this in a whiteboard and everything will make sense.
#[derive(Clone)]
struct BFNode {
    parent1: Option<Idx>,
    cost1: f64,

    parent2: Option<Idx>,
    cost2: f64,
}

impl BFNode {
    // Implementation of the logic mentioned in BFNode definition.
    fn get_proper_cost(&self, v: Idx) -> f64 {
        if self.parent1.is_some_and(|p| p == v) {
            self.cost2
        } else {
            self.cost1
        }
    }
}

impl Default for BFNode {
    fn default() -> Self {
        Self {
            parent1: None,
            cost1: f64::INFINITY,
            parent2: None,
            cost2: f64::INFINITY,
        }
    }
}

/// Modified `BellmanFord` algorithm
///
/// This function mixes these two parts of the algorithm:
///
/// ```text
///   (R, γ) := ComputeResidualGraph(G, c, f, ∆)
///     RunBellmanFord(R, γ)
/// ```
///
/// The `ComputeResidualGraph` returns the residual graph and then it is fullfiled with the
/// `RunBellmanFord` function. In this case, the function creates `R` (state) and returns it after
/// it ran the `BellmanFord` algorithm.
///
/// * `max_iters` is defined as 2 * n since it is shown that this is the maximum amount of
/// > iterations needed to complete this modified bellman ford (see in the paper).
fn bellman_ford(problem: &Problem, mask: &Mask, acc_flows: &[i8], delta: i8) -> Vec<BFNode> {
    let Problem { graph, .. } = problem;

    let n = graph.node_count();
    let substation = graph.substation();

    let mut state = vec![BFNode::default(); n];

    state[substation] = BFNode {
        parent1: None,
        cost1: 0.,
        parent2: None,
        cost2: 0.,
    };

    let max_iters = n * 2;

    for _ in 0..max_iters {
        let updated = relax_distances(problem, acc_flows, mask, &mut state, delta);

        if !updated {
            break;
        }
    }

    state
}

fn relax_distances(
    problem: &Problem,
    acc_flows: &[i8],
    mask: &Mask,
    state: &mut [BFNode],
    delta: i8,
) -> bool {
    let Problem { graph, .. } = problem;

    let mut updated = false;

    let n = graph.node_count();

    for u in graph.nodes() {
        for v in graph.nodes() {
            if u == v {
                continue;
            }

            let node = &state[u];

            let base_cost = node.get_proper_cost(v);
            let edge_cost = calculate_edge_cost(problem, u, v, acc_flows, mask, n, delta);
            let cost = base_cost + edge_cost;

            verify_promotion(&mut updated, state, u, v, cost);
        }
    }
    updated
}

fn verify_promotion(updated: &mut bool, state: &mut [BFNode], u: Idx, v: Idx, cost: f64) {
    if cost < state[v].cost1 {
        promote_best_cost(state, u, v, cost);
        *updated = true;
    } else if cost < state[v].cost2 && state[v].parent1 != Some(u) {
        promote_second_best(state, u, v, cost);
        *updated = true;
    }
}

fn promote_best_cost(state: &mut [BFNode], u: Idx, v: Idx, cost: f64) {
    if state[v].parent1 != Some(u) {
        state[v].cost2 = state[v].cost1;
        state[v].parent2 = state[v].parent1;
    }

    state[v].cost1 = cost;
    state[v].parent1 = Some(u);
}

fn promote_second_best(state: &mut [BFNode], u: Idx, v: Idx, cost: f64) {
    state[v].cost2 = cost;
    state[v].parent2 = Some(u);
}

// Optimization: pass u, v from previous iterations to find other closed walks
fn find_negative_closed_walk(
    problem: &Problem,
    r: &[BFNode],
    f: &[i8],
    mask: &Mask,
    delta: i8,
    (start_i, start_j): (usize, usize),
) -> Option<(Vec<Idx>, usize, usize)> {
    let Problem { graph, .. } = problem;

    let mut trigger_node = None;

    let n = graph.node_count();

    let mut next_start = (start_i, start_j);

    // Iterate over the edges (u,v) to find one node that can be relaxed.
    'outer: for from in start_i..n {
        let start_j = if from == start_i { start_j } else { 0 };

        for to in start_j..n {
            if from == to {
                continue;
            }

            let node_u = &r[from];
            let base_cost = node_u.get_proper_cost(to);

            if base_cost.is_infinite() {
                continue;
            }

            let edge_cost = calculate_edge_cost(problem, from, to, f, mask, n, delta);

            if base_cost + edge_cost < r[to].cost1 {
                trigger_node = Some(to);

                if to + 1 < n {
                    next_start = (from, to + 1);
                } else {
                    next_start = (from + 1, 0);
                }

                break 'outer;
            }
        }
    }

    let start_node = trigger_node?;

    let mut visited = vec![false; n];
    let mut traceback = Vec::new();
    let mut curr = start_node;

    loop {
        traceback.push(curr);
        visited[curr] = true;

        curr = r[curr].parent1?;

        if visited[curr] {
            traceback.push(curr);
            break;
        }
    }

    traceback.reverse();

    let root = traceback[0];
    let mut cycle = Vec::new();

    for &node in &traceback {
        cycle.push(node);
        if cycle.len() > 1 && node == root {
            break;
        }
    }

    Some((cycle, next_start.0, next_start.1))
}

fn decompose_walk_into_cycles(w: &[Idx], n: usize) -> Vec<Vec<Idx>> {
    let mut simple_cycles = Vec::new();
    let mut stack: Vec<Idx> = Vec::new();
    let mut in_stack = vec![false; n];

    for &node in w {
        if !in_stack[node] {
            stack.push(node);
            in_stack[node] = true;
            continue;
        }

        let mut cycle = Vec::new();
        cycle.push(node);

        while let Some(popped) = stack.pop() {
            in_stack[popped] = false;
            cycle.push(popped);
            if popped == node {
                break;
            }
        }
        cycle.reverse();
        simple_cycles.push(cycle);

        stack.push(node);
        in_stack[node] = true;
    }

    simple_cycles
}

fn add_flow_on_cycle(f: &mut [i8], mask: &mut Mask, cycle: &[Idx], n: usize, delta: i8) {
    for window in cycle.windows(2) {
        let u = window[0];
        let v = window[1];

        let dir_idx = utils::edge_to_idx(u, v, n);
        let inv_idx = utils::edge_to_idx(v, u, n);

        if f[inv_idx] > 0 {
            if delta <= f[inv_idx] {
                f[inv_idx] -= delta;
                if f[inv_idx] == 0 {
                    mask.remove_nodes(u, v);
                }
            } else {
                let remaining = delta - f[inv_idx];
                f[inv_idx] = 0;
                f[dir_idx] += remaining;
            }
        } else {
            if f[dir_idx] == 0 {
                mask.add_nodes(u, v);
            }
            f[dir_idx] += delta;
        }
    }
}

pub struct Delta {
    pub current: i8,
    highest_tested_since_change: i8,
    max: i8,
    is_decreasing: bool,
}

impl Delta {
    pub fn new(max: i8) -> Self {
        Self {
            current: 1,
            max,
            is_decreasing: false,
            highest_tested_since_change: 0,
        }
    }

    pub fn next(&mut self, cycle_cancelled: bool) -> bool {
        if cycle_cancelled {
            self.highest_tested_since_change = 0;
            self.is_decreasing = true;
        } else {
            self.highest_tested_since_change = self.highest_tested_since_change.max(self.current);
        }

        if self.highest_tested_since_change >= self.max {
            return false;
        }

        if self.is_decreasing {
            self.current -= 1;
            if self.current <= 1 {
                self.current = 1;
                self.is_decreasing = false;
            }
        } else {
            self.current += 1;
        }

        true
    }
}

fn solution_cost(problem: &Problem, flow: &[i8]) -> f64 {
    build_solution(problem, flow).cost
}
