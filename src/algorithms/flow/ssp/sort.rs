use crate::types::{Angle, Graph, Idx};
use std::{cmp::Ordering, fmt::Display};

#[derive(Clone, Debug, Copy)]
pub enum SSPSortStrategy {
    None,
    Nearest,
    Farthest,
    Sweep { band: f64 },
}

impl Display for SSPSortStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSPSortStrategy::None => write!(f, "SSP"),
            SSPSortStrategy::Nearest => write!(f, "SSP NEAREST"),
            SSPSortStrategy::Farthest => write!(f, "SSP FARTHEST"),
            SSPSortStrategy::Sweep { band } => write!(f, "SSP SWEEP ({:.2}%)", band * 100.),
        }
    }
}

impl SSPSortStrategy {
    /// Sorts the turbines from the problem acording to the strategy passed.
    ///
    /// It will return a sorted vector with only the turbines of the problem.
    pub(super) fn sort_and_filter_subs(self, graph: &Graph) -> Vec<Idx> {
        let mut turbines: Vec<Idx> = graph.nodes().filter(|&n| n != graph.substation()).collect();

        let substation = graph.substation();

        match self {
            SSPSortStrategy::Nearest => {
                turbines.sort_by(|&a, &b| {
                    let distance_a = graph[a][substation];
                    let distance_b = graph[b][substation];
                    distance_a
                        .partial_cmp(&distance_b)
                        .unwrap_or(Ordering::Equal)
                });
            }
            SSPSortStrategy::Farthest => {
                turbines.sort_by(|&a, &b| {
                    let dist_a = graph[a][substation];
                    let dist_b = graph[b][substation];
                    dist_b.partial_cmp(&dist_a).unwrap_or(Ordering::Equal)
                });
            }
            SSPSortStrategy::Sweep { band } => {
                turbines = Self::sort_by_polar_and_sweep(graph, band);
            }
            SSPSortStrategy::None => {}
        }

        turbines
    }

    fn sort_by_polar_and_sweep(graph: &Graph, band_percent: f64) -> Vec<Idx> {
        let substation = graph.substation();
        let substation_node = graph.node(substation);

        let mut turbines_info: Vec<(Idx, f64, Angle)> = graph
            .nodes()
            .filter(|&n| n != substation)
            .map(|idx| {
                let dist = graph[idx][substation];
                let node = graph.node(idx);
                let angle = (node.y - substation_node.y).atan2(node.x - substation_node.x);
                (idx, dist, angle)
            })
            .collect();

        if turbines_info.is_empty() {
            return Vec::new();
        }

        let max_dist = turbines_info
            .iter()
            .map(|&(_, d, _)| d)
            .fold(0.0_f64, f64::max);

        let min_dist = turbines_info
            .iter()
            .map(|&(_, d, _)| d)
            .fold(f64::INFINITY, f64::min);

        let dist_range = max_dist - min_dist;

        let band_thickness = if dist_range > 0.0 {
            dist_range * band_percent
        } else {
            1.0
        };

        turbines_info.sort_by(|&(_, dist_a, angle_a), &(_, dist_b, angle_b)| {
            let ring_a = ((dist_a - min_dist) / band_thickness).floor() as i32;
            let ring_b = ((dist_b - min_dist) / band_thickness).floor() as i32;

            let ring_cmp = ring_a.cmp(&ring_b);

            if ring_cmp != Ordering::Equal {
                return ring_cmp;
            }

            angle_a.partial_cmp(&angle_b).unwrap_or(Ordering::Equal)
        });

        turbines_info.into_iter().map(|(idx, _, _)| idx).collect()
    }
}
