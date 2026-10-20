use crate::{
    algorithms::{
        mecanism,
        sweep::{self, Direction, initialize_groups},
        transgenetic::vetor::{PartialSolution, TransgeneticVector},
    },
    types::{Idx, Problem, Solution},
};
use rand::{RngExt, rngs::StdRng};

/// Transposon baseado em setores angulares.
///
/// Destrói uma janela angular (topograficamente coerente) de turbinas e a reconstrói com o
/// sweeping (multistart limitado dentro do setor, ancorado na subestação), seguida de busca local
/// restrita às arestas do setor.
pub struct SectorTransposon {
    /// Fração das turbinas que forma o setor destruído.
    pub sector_size: f64,
}

impl TransgeneticVector for SectorTransposon {
    fn transcribe(
        &self,
        problem: &Problem,
        receptor: &Solution,
        rng: &mut StdRng,
        iters: &mut usize,
    ) -> Solution {
        let substation = problem.graph.substation();

        // 1. Setor angular (janela contígua em ordem polar).
        let sorted = mecanism::sort_turbines_by_angle(&problem.graph);
        let n = sorted.len();
        let sector_factor = (self.sector_size + rng.random_range(-0.3..=0.3)).clamp(0.0, 1.0);
        let w = ((sector_factor * n as f64).round() as usize).clamp(1, n);
        let start = rng.random_range(0..n);
        let sector_sorted: Vec<(Idx, f64)> = (0..w).map(|i| sorted[(start + i) % n]).collect();
        let sector: Vec<Idx> = sector_sorted.iter().map(|&(idx, _)| idx).collect();

        // 2. Destrói o setor, congela o resto.
        let mut partial: PartialSolution = receptor.into();
        partial.destroy(&sector, substation);
        let base_parents: Vec<Idx> = partial
            .parents
            .iter()
            .map(|p| p.unwrap_or(substation))
            .collect();
        let seed_mask = &partial.mask;

        let limit = problem.graph.substation_limit();
        let min_tpg = w.div_ceil(limit as usize).max(1);
        let max_tpg = (problem.cables.best_cable_conn() as usize).max(min_tpg);

        let mut best: Option<Solution> = None;

        for start_idx in 0..sector.len() {
            for dir in &[Direction::Clockwise, Direction::CounterClockwise] {
                for tpg in min_tpg..=max_tpg {
                    #[allow(clippy::cast_possible_wrap)]
                    let groups =
                        sweep::initialize_groups(&sector_sorted, *dir, start_idx as isize, tpg);

                    let mut cand = base_parents.clone();
                    for g in &groups {
                        mecanism::prim_seeded(problem, g, &mut cand, seed_mask);
                    }

                    let mut sol = Solution::from_parents(cand, problem);
                    mecanism::evaluate(problem, &mut sol);

                    *iters += 1;

                    if best.as_ref().is_none_or(|b| sol.cost < b.cost) {
                        best = Some(sol);
                    }
                }
            }
        }

        let mut solution = best.expect("multistart sempre produz ao menos uma solução");
        *iters += mecanism::local_search(problem, &mut solution);

        solution
    }
}
