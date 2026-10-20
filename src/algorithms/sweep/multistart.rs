use crate::{
    algorithms::{
        Algorithm, mecanism,
        sweep::{self, Direction},
    },
    types::{Problem, Solution},
};
use tracing::info;

#[derive(Clone, Copy)]
pub struct SweepingMultiStart;

impl Algorithm for SweepingMultiStart {
    fn name(&self) -> String {
        String::from("SweepingMultistart")
    }

    fn clone_box(&self) -> Box<dyn Algorithm> {
        Box::new(*self)
    }

    /// Sweeping Multi-start implementation.
    ///
    /// The algorithm sorts all turbines by their angle relative to the substation.
    /// It then creates groups ranging from a minimum size (Total Turbines / Substation Inputs)
    /// up to a maximum size (Capacity of the largest cable).
    ///
    /// For each possible group size, starting position, and direction, it groups
    /// the turbines based on the sorted angles.
    ///
    /// For each group found in a given start configuration, it runs Prim's algorithm
    /// (rooted at the substation) to build the MST. Finally, it uses a DFS traversal
    /// to calculate the total cost, returning the best solution found.
    fn execute(&mut self, problem: &Problem) -> Solution {
        let Problem { graph, cables, .. } = problem;

        // Maximum number of direct connections the substation can handle.
        let max_connections = graph.substation_limit();

        let turbines = mecanism::sort_turbines_by_angle(graph);

        #[allow(clippy::cast_possible_wrap)]
        let n = turbines.len();

        assert!(isize::try_from(n).is_ok());

        // TPG = Turbines per group.
        assert!(max_connections > 0);
        #[allow(clippy::cast_sign_loss)]
        let min_tpg = (n).div_ceil(max_connections as usize);

        assert!(cables.best_cable_conn() > 0);
        #[allow(clippy::cast_sign_loss)]
        let max_tpg = cables.best_cable_conn() as usize;

        let tpg_count = if max_tpg >= min_tpg {
            max_tpg - min_tpg + 1
        } else {
            0
        };
        info!(
            "SweepingMultiStart: n={} tpg {}..={} ({} configs)",
            n,
            min_tpg,
            max_tpg,
            n * 2 * tpg_count
        );

        // The best configuration found so far.
        // Starts with the worse possible cost (f64::INFINITY)
        let mut best_soluton = Solution::new(problem);
        best_soluton.cost = f64::INFINITY;

        for start_idx in 0..n {
            for dir in &[Direction::Clockwise, Direction::CounterClockwise] {
                for tpg in min_tpg..=max_tpg {
                    #[allow(clippy::cast_possible_wrap)]
                    let groups = sweep::initialize_groups(&turbines, *dir, start_idx as isize, tpg);

                    let current_solution = sweep::evaluate_solution(problem, groups);

                    // Update if a lower-cost solution is found.
                    if current_solution.cost < best_soluton.cost {
                        best_soluton = current_solution;
                    }
                }
            }
        }

        best_soluton
    }
}
