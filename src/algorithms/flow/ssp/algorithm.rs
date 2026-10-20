use std::collections::BinaryHeap;

use crate::algorithms::flow::{build_solution, calculate_edge_cost};
use crate::algorithms::{Algorithm, Ssp};
use crate::types::{Idx, Mask, Problem, Solution, State};
use crate::utils;

impl Algorithm for Ssp {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn clone_box(&self) -> Box<dyn Algorithm> {
        Box::new(self.clone())
    }

    fn execute(&mut self, problem: &Problem) -> Solution {
        let graph = &problem.graph;
        let substation = graph.substation();

        let turbines = self.sort_strategy.sort_and_filter_subs(graph);

        let mut acc_flows: Vec<i8> = vec![0; graph.node_count().pow(2)];
        let n = graph.node_count();

        let mut mask = Mask::new(problem.colisions.blocks_per_row);

        for turbine in turbines {
            // Get the current shortest path
            let path = self.dikstra(problem, turbine, substation, &acc_flows, &mask);

            for i in 0..(path.len() - 1) {
                let u = path[i];
                let v = path[i + 1];

                let dir_idx = utils::edge_to_idx(u, v, n);
                let inv_idx = utils::edge_to_idx(v, u, n);

                let inv_flow = acc_flows[inv_idx];

                if inv_flow > 0 {
                    acc_flows[inv_idx] -= 1;
                } else {
                    acc_flows[dir_idx] += 1;
                }

                if acc_flows[inv_idx] != 0 || acc_flows[dir_idx] != 0 {
                    mask.add_nodes(u, v);
                } else {
                    mask.remove_nodes(u, v);
                }
            }
        }

        build_solution(problem, &acc_flows)
    }
}

impl Ssp {
    /// Dikstra algorithm
    ///
    /// Dikstra implementation with the limitation of making it going into any vertex at most one time.
    #[inline]
    fn dikstra(
        &self,
        problem: &Problem,
        start: Idx,
        target: Idx,
        active_flows: &[i8],
        mask: &Mask,
    ) -> Vec<Idx> {
        let graph = &problem.graph;
        let node_count = graph.node_count();

        // Distances list, start with infinity cost to every node, except for the start index which has
        // no cost.
        let mut distances = vec![f64::INFINITY; node_count];
        distances[start] = 0.0;

        // Predecessors to create a map to track the path, something like: A -> C -> D -> X -> Z
        let mut predecessors = vec![None; node_count];

        // Limitation of the algorithm, we can visit only one node at most.
        let mut visited = vec![false; node_count];

        // heap to visit the nodes. We also start by `start` node with zero cost.
        let mut heap = BinaryHeap::new();
        heap.push(State {
            cost: 0.0,
            node: start,
        });

        while let Some(State { cost, node }) = heap.pop() {
            // If node was visited, skip this iteration.
            if visited[node] {
                continue;
            }
            visited[node] = true;

            // If we reach the target, break.
            if node == target {
                break;
            }

            // Since we are working with a complete graph, evaluate the cost for every other node.
            // (Could be improved by not looking to everyone.)
            for next_node in graph.nodes() {
                if next_node == node || visited[next_node] {
                    continue;
                }

                if forms_bifurcation(node, next_node, node_count, active_flows) {
                    continue;
                }

                let upgrade_cost = calculate_edge_cost(
                    problem,
                    node,
                    next_node,
                    active_flows,
                    mask,
                    node_count,
                    1,
                );
                let next_cost = cost + upgrade_cost;

                relax_edge(
                    node,
                    next_node,
                    next_cost,
                    &mut distances,
                    &mut predecessors,
                    &mut heap,
                );
            }
        }

        // If didn't find any path, there is something wrong.
        assert!(
            distances[target] != f64::INFINITY,
            "Shortest path wasn't found."
        );

        reconstruct_path(&predecessors, target)
    }
}

/// Evaluates if the newly calculated cost to a node is better than the currently known cost.
/// If so, updates the shortest distance, records the predecessor, and adds the node to the evaluation heap.
/// This process is standardly known as edge relaxation.
///
/// # Arguments
///
/// * `node` - The current node we are expanding from.
/// * `next_node` - The target node we are evaluating.
/// * `next_cost` - The calculated cost to reach `next_node` via `node`.
/// * `distances` - Mutable slice tracking the shortest known distance to each node.
/// * `predecessors` - Mutable slice tracking the optimal path back to the start.
/// * `heap` - Mutable reference to the priority queue managing the nodes to visit.
#[inline]
fn relax_edge(
    node: Idx,
    next_node: Idx,
    next_cost: f64,
    distances: &mut [f64],
    predecessors: &mut [Option<Idx>],
    heap: &mut BinaryHeap<State>,
) {
    if next_cost < distances[next_node] {
        distances[next_node] = next_cost;
        predecessors[next_node] = Some(node);
        heap.push(State {
            cost: next_cost,
            node: next_node,
        });
    }
}

fn reconstruct_path(predecessors: &[Option<Idx>], target: Idx) -> Vec<Idx> {
    let mut path = Vec::new();
    let mut curr = target;
    path.push(curr);

    while let Some(prev) = predecessors[curr] {
        curr = prev;
        path.push(curr);
    }
    path.reverse();
    path
}

/// Evaluates if connecting the current node to the next node creates a flow bifurcation.
///
/// # Arguments
///
/// * `node` - The current node being evaluated by the routing algorithm.
/// * `next_node` - The prospective destination node to connect to.
/// * `active_flows` - A slice representing the current network state.
/// * `total_number_nodes` - The total number of nodes in the graph.
///
/// # Returns
///
/// * `bool` - Returns `true` if connecting to `next_node` violates the single out-degree rule.
#[inline]
fn forms_bifurcation(
    source_node: Idx,
    next_node: Idx,
    total_number_nodes: usize,
    active_flows: &[i8],
) -> bool {
    for target_node in 0..total_number_nodes {
        if has_conflicting_out_flow(
            source_node,
            next_node,
            target_node,
            total_number_nodes,
            active_flows,
        ) {
            return true;
        }
    }

    false
}

/// Checks if there is a conflicting outgoing flow from the current node.
///
/// Evaluates if the `node` already has an active cable sending power to a `target_node`,
/// assuming this target is neither the current node itself nor the intended `next_node`.
///
/// # Arguments
///
/// * `node` - The source node currently being evaluated.
/// * `next_node` - The prospective destination for the new route.
/// * `target_node` - The node being checked for existing flow in the current loop iteration.
/// * `total_number_nodes` - The total number of vertices in the graph.
/// * `active_flows` - A 1D slice representing the current state of network flows.
///
/// # Returns
///
/// * `bool` - Returns `true` if there is a flow greater than zero indicating a bifurcation.#[inline]
fn has_conflicting_out_flow(
    node: Idx,
    next_node: Idx,
    target_node: Idx,
    total_number_nodes: usize,
    active_flows: &[i8],
) -> bool {
    if target_node != next_node && target_node != node {
        let edge_idx = utils::edge_to_idx(node, target_node, total_number_nodes);

        if active_flows[edge_idx] > 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reconstruct_path_preserves_order_from_source_to_destination() {
        // path: 1 -> 4 -> 3 -> 5
        let predecessors = vec![None, None, None, Some(4), Some(1), Some(3)];

        let target = 5;

        let path = reconstruct_path(&predecessors, target);

        let expected = vec![1, 4, 3, 5];

        assert_eq!(path, expected);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn reconstruct_path_panics_if_target_out_of_bounds() {
        let predecessors = vec![None; 2];

        let _path = reconstruct_path(&predecessors, 5);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn reconstruct_path_panics_if_intermediate_predecessor_out_of_bounds() {
        let mut predecessors = vec![None; 2];
        predecessors[1] = Some(99);

        let _path = reconstruct_path(&predecessors, 1);
    }
}
