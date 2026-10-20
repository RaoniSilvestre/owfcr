use crate::{
    algorithms::{
        Algorithm, mecanism,
        sweep::{self, Direction},
    },
    types::{Problem, Solution},
};

#[derive(Clone, Copy)]
pub struct Sweeping;

impl Algorithm for Sweeping {
    fn name(&self) -> String {
        String::from("Sweeping")
    }

    fn clone_box(&self) -> Box<dyn Algorithm> {
        Box::new(*self)
    }

    /// Sweeping heuristic implementation
    ///
    /// The main idea is to order all the turbines by their relative angle with respect to the
    /// substation and cluster them by that order.
    ///
    /// After the group formation phase, for each group, run prim's algorithm (find the MST)
    /// considering only the group.
    ///
    /// With all the MST's formed, evaluate the total solution.
    fn execute(&mut self, problem: &Problem) -> Solution {
        let Problem { graph, .. } = problem;

        let turbines = mecanism::sort_turbines_by_angle(graph);

        let max_connections = graph.substation_limit();
        let n_turbines = turbines.len();

        // The three variables below could be passed as arguments to see changes in behaviour, but
        // for simplicity it is all hardcoded together here.

        // TPG = Turbines per group. In this case it is set to the max value possible.

        assert!(max_connections > 0);
        assert!(n_turbines > 0);

        #[allow(clippy::cast_sign_loss)]
        let tpg = (n_turbines as f64 / f64::from(max_connections)).ceil() as usize;
        // Start at the left/rightmost turbine
        let start_idx = 0;
        // And go in that direction until all the turbines were passed.
        let dir = Direction::Clockwise;

        // Initialization of groups.
        let groups = sweep::initialize_groups(&turbines, dir, start_idx, tpg);

        sweep::evaluate_solution(problem, groups)
    }
}
