use std::ops::{Index, Range};

use crate::types::WindFarmNode;

#[derive(Clone)]
pub struct Graph {
    nodes: Vec<WindFarmNode>,
    edges: Vec<f64>,
    substation: usize,
}

impl Graph {
    pub fn new(nodes: Vec<WindFarmNode>) -> Self {
        let n = nodes.len();

        let mut edges = vec![0.; n * n];

        let mut substation = 0;

        for (i, u) in nodes.iter().enumerate() {
            if u.w < 0 {
                substation = i;
            }

            for (j, v) in nodes.iter().enumerate() {
                if i == j {
                    continue;
                }

                let uv = edge_to_idx(i, j, n);

                let d = ((u.x - v.x).powf(2.0) + (u.y - v.y).powf(2.0)).sqrt();
                edges[uv] = d;
            }
        }

        Self {
            nodes,
            edges,
            substation,
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn node(&self, i: usize) -> WindFarmNode {
        self.nodes[i]
    }

    pub fn nodes(&self) -> Range<usize> {
        Range {
            start: 0,
            end: self.node_count(),
        }
    }

    pub fn substation(&self) -> usize {
        self.substation
    }

    pub fn substation_limit(&self) -> i8 {
        -self.nodes[self.substation].w
    }
}

impl Index<usize> for Graph {
    type Output = [f64];

    fn index(&self, row: usize) -> &Self::Output {
        let n = self.nodes.len();
        &self.edges[row * n..(row + 1) * n]
    }
}

/// Flattens a (u, v) vertex pair into a one dimension index.
///
/// It is the inverse operation of `get_edge`.
#[inline]
fn edge_to_idx(u: usize, v: usize, n: usize) -> usize {
    u * n + v
}
