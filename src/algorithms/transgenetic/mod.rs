use crate::{
    algorithms::{
        Algorithm, mecanism,
        transgenetic::vetor::{Plasmid, SectorTransposon, TransgeneticVector},
    },
    log,
    types::{Problem, Solution},
};
use rand::{RngExt, SeedableRng, rngs::StdRng, seq::IndexedRandom};
use std::time::Instant;
use tracing::instrument;

mod vetor;

/// Transgenetic algorithm: population metaheuristic inspired by horizontal gene transfer.
pub struct Transgenetic {
    pub name: String,
    pub algorithm: Vec<Box<dyn Algorithm>>,
    pub max_iterations: usize,
    pub seed: u64,
    pub iters: usize,
    pub population_size: usize,
    /// Fraction of the population considered elite (plasmid donors / material source).
    pub elite_ratio: f64,
    /// Lower bound of the transposon subtree size, as a fraction of the solution.
    pub plasmid_size_min: f64,
    /// Upper bound of the transposon subtree size, as a fraction of the solution.
    pub plasmid_size_max: f64,
    /// Number of plasmids kept in the precomputed bank.
    pub plasmid_bank_size: usize,
    /// Fração de turbinas destruídas pelo setor do setor transposon.
    pub sector_size: f64,
    /// Número de configurações amostradas pelo multistart do setor transposon.
    pub multistart_configs: usize,
    /// Fração mínima de nós destruídos pelo vetor Destroy & Repair.
    pub destroy_repair_min: f64,
    /// Fração máxima de nós destruídos pelo vetor Destroy & Repair.
    pub destroy_repair_max: f64,
    /// Probabilidade relativa de cada vetor, na ordem:
    /// `[plasmid, sector]`. Normalizado internamente.
    pub vector_weights: [f64; 2],
}

impl Algorithm for Transgenetic {
    fn name(&self) -> String {
        format!(
            "{} (random intense sector transposson) ({} iters)",
            self.name, self.max_iterations
        )
    }

    fn clone_box(&self) -> Box<dyn Algorithm> {
        Box::new(self.clone())
    }

    #[instrument(skip_all, name = "Transgenetic")]
    #[allow(clippy::cast_sign_loss)]
    fn execute(&mut self, problem: &Problem) -> Solution {
        let Initialization {
            mut plasmid_bank,
            mut chromossomes,
            mut rng,
        } = self.initialize(problem);

        let sector_transposon = SectorTransposon {
            sector_size: self.sector_size,
        };

        let initial_solution = chromossomes[0].clone();
        let mut last_log_time = Instant::now();

        let weights = self.vector_weights;
        let bank_refresh_interval = (self.max_iterations / 10).max(1);
        let mut next_bank_refresh = bank_refresh_interval;

        while self.iters < self.max_iterations {
            for s in &mut chromossomes {
                if self.iters >= self.max_iterations {
                    break;
                }

                let kind = sample_kind(&mut rng, &weights);

                let transcription = match kind {
                    Kind::Plasmid => plasmid_bank.choose(&mut rng).unwrap().transcribed(
                        problem,
                        s,
                        &mut rng,
                        &mut self.iters,
                    ),
                    Kind::Sector => {
                        sector_transposon.transcribed(problem, s, &mut rng, &mut self.iters)
                    }
                };

                let Some(improved) = transcription else {
                    continue;
                };

                *s = improved;
            }

            chromossomes.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());

            if self.iters >= next_bank_refresh {
                next_bank_refresh += bank_refresh_interval;
                self.refresh_plasmids(problem, &mut plasmid_bank, &chromossomes, &mut rng);
            }

            log::log_progress(
                self.iters,
                self.max_iterations,
                &mut last_log_time,
                chromossomes[0].cost,
                initial_solution.cost,
            );
        }

        log::log_result(
            self.iters,
            self.max_iterations,
            chromossomes[0].cost,
            initial_solution.cost,
        );

        chromossomes.remove(0)
    }
}

struct Initialization {
    plasmid_bank: Vec<Plasmid>,
    chromossomes: Vec<Solution>,
    rng: StdRng,
}

/// Vetor transgenético escolhido em uma iteração.
#[derive(Clone, Copy)]
enum Kind {
    Plasmid,
    Sector,
}

impl Kind {
    const fn from_index(i: usize) -> Self {
        match i {
            0 => Kind::Plasmid,
            _ => Kind::Sector,
        }
    }
}

/// Sorteia um vetor proporcionalmente aos pesos.
fn sample_kind(rng: &mut StdRng, weights: &[f64; 2]) -> Kind {
    let total: f64 = weights.iter().sum();

    let mut r = rng.random::<f64>() * total;

    for (i, &w) in weights.iter().enumerate() {
        if r < w {
            return Kind::from_index(i);
        }
        r -= w;
    }

    Kind::Sector
}

impl Clone for Transgenetic {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            algorithm: self.algorithm.iter().map(|a| a.clone_box()).collect(),
            max_iterations: self.max_iterations,
            seed: self.seed,
            iters: self.iters,
            population_size: self.population_size,
            elite_ratio: self.elite_ratio,
            plasmid_size_min: self.plasmid_size_min,
            plasmid_size_max: self.plasmid_size_max,
            plasmid_bank_size: self.plasmid_bank_size,
            sector_size: self.sector_size,
            multistart_configs: self.multistart_configs,
            destroy_repair_min: self.destroy_repair_min,
            destroy_repair_max: self.destroy_repair_max,
            vector_weights: self.vector_weights,
        }
    }
}

impl Transgenetic {
    fn elite_size(&self) -> usize {
        let ratio = self.elite_ratio * self.population_size as f64;
        let rounded = ratio.round() as usize;

        rounded.clamp(1, self.population_size)
    }

    fn refresh_plasmids(
        &self,
        problem: &Problem,
        bank: &mut Vec<Plasmid>,
        chromossomes: &[Solution],
        rng: &mut StdRng,
    ) {
        let elite_size = self.elite_size();
        let size_range = (self.plasmid_size_min, self.plasmid_size_max);

        *bank = Plasmid::generate_bank(
            problem,
            &chromossomes[..elite_size],
            self.plasmid_bank_size,
            size_range,
            rng,
        );
    }

    fn initialize(&mut self, problem: &Problem) -> Initialization {
        self.iters = 0;
        let mut rng = StdRng::seed_from_u64(self.seed);

        let chromossomes = mecanism::populate(
            &mut self.algorithm,
            self.population_size,
            10,
            problem,
            &mut rng,
        );
        let n = chromossomes.len();

        let elite_size = ((self.elite_ratio * n as f64).round() as usize).clamp(1, n);
        let size_range = (self.plasmid_size_min, self.plasmid_size_max);

        let plasmid_bank = Plasmid::generate_bank(
            problem,
            &chromossomes[..elite_size],
            self.plasmid_bank_size,
            size_range,
            &mut rng,
        );

        Initialization {
            plasmid_bank,
            chromossomes,
            rng,
        }
    }
}
