#![allow(unused)]

use crate::{
    types::{Graph, Idx},
    utils::{self, edge_id},
};
use fixedbitset::FixedBitSet;

#[derive(Clone)]
pub struct BasicIntersectionTester {
    colisions: FixedBitSet,
    edge_count: usize,
}

impl BasicIntersectionTester {
    #[must_use]
    pub fn new(g: &Graph) -> Self {
        let n = g.node_count();
        let max_edges = (n * (n - 1)) / 2;

        let mut colisions = FixedBitSet::with_capacity(max_edges * max_edges);

        // 4 loops: para cada aresta (u1 -> v1) e cada aresta (u2 -> v2)
        for u1 in 0..n {
            for v1 in 0..n {
                if u1 == v1 {
                    continue;
                }
                let e1id = utils::edge_to_idx(u1, v1, n);

                for u2 in 0..n {
                    for v2 in 0..n {
                        if u2 == v2 {
                            continue;
                        }
                        let e2id = utils::edge_to_idx(u2, v2, n);

                        if e1id == e2id {
                            continue;
                        }

                        if super::edges_intersect(g, u1, v1, u2, v2) {
                            colisions.insert(e1id * max_edges + e2id);
                        }
                    }
                }
            }
        }

        Self {
            colisions,
            edge_count: max_edges,
        }
    }

    #[must_use]
    pub fn intersect(&self, u1: Idx, v1: Idx, u2: Idx, v2: Idx) -> bool {
        let e1id = edge_id(u1, v1);
        let e2id = edge_id(u2, v2);

        self.colisions[e1id * self.edge_count + e2id]
    }
}
