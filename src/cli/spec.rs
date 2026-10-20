use crate::algorithms::{
    Algorithm, Genetic, Memetic, NegativeCycleCanceling, RandomAlgorithm, SSPSortStrategy, Ssp,
    Sweeping, SweepingMultiStart, Transgenetic, VariableNeighborhoodSearch,
};
use color_eyre::eyre::Result;
use serde::Deserialize;
use tracing::info;

// 1. Enum único para descrever qualquer nó na árvore de algoritmos
#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AlgorithmSpec {
    /// Run SSP
    Ssp { tipo: SSPStrat, band: Option<f64> },
    /// Run Sweeping
    Sweeping,
    /// Run Sweeping Multi-start
    SweepingMultiStart,
    /// Generate a completely random solution
    Random {
        /// If not set, a random seed is chosen for each execution.
        seed: Option<u64>,
    },
    /// Run Variable Neighbourhood Search over an inner algorithm
    Vns {
        iterations: usize,

        /// If not set, a random seed is chosen for each execution.
        seed: Option<u64>,

        #[serde(default = "default_k_max")]
        k_max: i32,

        inner: Option<Box<AlgorithmSpec>>,
    },
    /// Run Negative Cycle Canceling over an inner algorithm
    Ncc { inner: Option<Box<AlgorithmSpec>> },

    Memetic {
        iterations: usize,

        /// If not set, a random seed is chosen for each execution.
        seed: Option<u64>,

        #[serde(default = "default_k_max")]
        k_max: i32,

        population_size: usize,

        algorithms: Vec<AlgorithmSpec>,
    },

    Genetic {
        iterations: usize,

        /// If not set, a random seed is chosen for each execution.
        seed: Option<u64>,

        #[serde(default = "default_k_max")]
        k_max: i32,

        population_size: usize,

        algorithms: Vec<AlgorithmSpec>,
    },

    Transgenetic {
        iterations: usize,

        /// If not set, a random seed is chosen for each execution.
        seed: Option<u64>,

        population_size: usize,

        #[serde(default = "default_elite_ratio")]
        elite_ratio: f64,

        #[serde(default = "default_plasmid_size_min")]
        plasmid_size_min: f64,

        #[serde(default = "default_plasmid_size_max")]
        plasmid_size_max: f64,

        #[serde(default = "default_plasmid_bank_size")]
        plasmid_bank_size: usize,

        #[serde(default = "default_sector_size")]
        sector_size: f64,

        #[serde(default = "default_multistart_configs")]
        multistart_configs: usize,

        #[serde(default = "default_destroy_repair_min")]
        destroy_repair_min: f64,

        #[serde(default = "default_destroy_repair_max")]
        destroy_repair_max: f64,

        #[serde(default = "default_vector_weights")]
        vector_weights: [f64; 2],

        algorithms: Vec<AlgorithmSpec>,
    },
}

// 2. Método de construção recursivo e limpo
impl TryFrom<AlgorithmSpec> for Box<dyn Algorithm> {
    type Error = color_eyre::eyre::Report;

    fn try_from(spec: AlgorithmSpec) -> Result<Self, Self::Error> {
        info!("Inicializando algoritmo {:?}", spec);
        let alg: Box<dyn Algorithm> = match spec {
            AlgorithmSpec::Ssp { tipo, band } => {
                let strategy = match tipo {
                    SSPStrat::Normal => SSPSortStrategy::None,
                    SSPStrat::Nearest => SSPSortStrategy::Nearest,
                    SSPStrat::Fartherst => SSPSortStrategy::Farthest,
                    SSPStrat::Sweep => SSPSortStrategy::Sweep {
                        band: band.unwrap_or(0.1),
                    },
                };
                Box::new(Ssp::new(strategy))
            }
            AlgorithmSpec::Sweeping => Box::new(Sweeping),
            AlgorithmSpec::SweepingMultiStart => Box::new(SweepingMultiStart),
            AlgorithmSpec::Random { seed } => {
                Box::new(RandomAlgorithm::new(seed.unwrap_or_else(rand::random)))
            }
            AlgorithmSpec::Vns {
                iterations,
                seed,
                k_max,
                inner,
            } => {
                let inner_alg = inner
                    .map(|b| Box::<dyn Algorithm>::try_from(*b))
                    .transpose()?
                    .unwrap_or_else(|| Box::new(SweepingMultiStart));

                let name = format!("VNS + {}", inner_alg.name());
                Box::new(VariableNeighborhoodSearch::new(
                    name,
                    iterations,
                    seed.unwrap_or_else(rand::random),
                    inner_alg,
                    k_max,
                ))
            }
            AlgorithmSpec::Ncc { inner } => {
                let inner_alg = inner
                    .map(|b| Box::<dyn Algorithm>::try_from(*b))
                    .transpose()?
                    .unwrap_or_else(|| Box::new(Ssp::new(SSPSortStrategy::None)));

                let name = format!("NCC + {}", inner_alg.name());
                Box::new(NegativeCycleCanceling::new(name, inner_alg))
            }
            AlgorithmSpec::Memetic {
                iterations,
                seed,
                k_max,
                population_size,
                algorithms,
            } => {
                let mut algs = Vec::with_capacity(algorithms.len());
                for spec in algorithms {
                    algs.push(Box::<dyn Algorithm>::try_from(spec)?);
                }

                Box::new(Memetic {
                    name: format!("Memetic (Pop: {population_size})"),
                    algorithm: algs,
                    max_iterations: iterations,
                    seed: seed.unwrap_or_else(rand::random),
                    k_max,
                    iters: 0,
                    population_size,
                })
            }
            AlgorithmSpec::Genetic {
                iterations,
                seed,
                k_max,
                population_size,
                algorithms,
            } => {
                let mut algs = Vec::with_capacity(algorithms.len());
                for spec in algorithms {
                    algs.push(Box::<dyn Algorithm>::try_from(spec)?);
                }

                Box::new(Genetic {
                    name: format!("Genetic (Pop: {population_size})"),
                    algorithm: algs,
                    max_iterations: iterations,
                    seed: seed.unwrap_or_else(rand::random),
                    k_max,
                    iters: 0,
                    population_size,
                })
            }
            AlgorithmSpec::Transgenetic {
                iterations,
                seed,
                population_size,
                elite_ratio,
                plasmid_size_min,
                plasmid_size_max,
                plasmid_bank_size,
                sector_size,
                multistart_configs,
                destroy_repair_min,
                destroy_repair_max,
                vector_weights,
                algorithms,
            } => {
                let mut algs = Vec::with_capacity(algorithms.len());
                for spec in algorithms {
                    algs.push(Box::<dyn Algorithm>::try_from(spec)?);
                }

                Box::new(Transgenetic {
                    name: format!("Transgenetic (Pop: {population_size})"),
                    algorithm: algs,
                    max_iterations: iterations,
                    seed: seed.unwrap_or_else(rand::random),
                    iters: 0,
                    population_size,
                    elite_ratio,
                    plasmid_size_min,
                    plasmid_size_max,
                    plasmid_bank_size,
                    sector_size,
                    multistart_configs,
                    destroy_repair_min,
                    destroy_repair_max,
                    vector_weights,
                })
            }
        };

        info!("Algoritmo inicializado. {:?}", alg.name());

        Ok(alg)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SSPStrat {
    /// No special ordering
    Normal,
    /// Euclidean distance, from nearest to fartherst
    Nearest,
    /// Euclidean distance, from fartherst to nearest
    Fartherst,
    /// Polar sweep order by distance bands
    Sweep,
}

const fn default_k_max() -> i32 {
    5
}

const fn default_elite_ratio() -> f64 {
    0.3
}

const fn default_plasmid_size_min() -> f64 {
    0.30
}

const fn default_plasmid_size_max() -> f64 {
    0.60
}

const fn default_plasmid_bank_size() -> usize {
    90
}

const fn default_sector_size() -> f64 {
    0.2
}

const fn default_multistart_configs() -> usize {
    8
}

const fn default_destroy_repair_min() -> f64 {
    0.4
}

const fn default_destroy_repair_max() -> f64 {
    0.6
}

const fn default_vector_weights() -> [f64; 2] {
    [0.5, 0.5]
}
