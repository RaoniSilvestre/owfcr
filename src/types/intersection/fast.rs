use crate::{
    types::{Graph, Idx},
    utils::{self, edge_id},
};
use std::simd::num::SimdUint;
use std::simd::u64x4;

/// Intersection checker.
#[derive(Clone)]
pub struct FastIntersectionTester {
    /// Bit matrix of the edges. For indexing it uses a pairing function.
    matrix: Vec<u64>,

    /// Blocks for each row of the matrix.
    pub blocks_per_row: usize,
}

impl FastIntersectionTester {
    /// Initializes all the intersections in a graph, optimizing every 64 flags of the "matrix" in a
    /// single u64. If there is less than 64 edges, then it will use 1 u64 for every row, if there
    /// is 150 edges, it will use 3 u64 (3 * 64 > 150 > 2 * 64).
    ///
    /// Optimizing space and enabling verification with boolean operators, enabling SIMD operations.
    #[must_use]
    pub fn new(g: &Graph) -> Self {
        let n = g.node_count();
        let max_edges = utils::max_edges(g); // ou n * n
        let blocks_per_row = utils::block_size(g);

        let mut matrix = vec![0u64; max_edges * blocks_per_row];

        // 4 loops: para cada aresta (u1 -> v1) e cada aresta (u2 -> v2)
        for u1 in 0..n {
            for v1 in 0..n {
                if u1 == v1 {
                    continue;
                }
                let e1id = utils::edge_id(u1, v1);

                for u2 in 0..n {
                    for v2 in 0..n {
                        if u2 == v2 {
                            continue;
                        }
                        let e2id = utils::edge_id(u2, v2);

                        if e1id == e2id {
                            continue;
                        }

                        if super::edges_intersect(g, u1, v1, u2, v2) {
                            let block_idx = (e1id * blocks_per_row) + (e2id / 64);
                            let bit_idx = e2id % 64;
                            matrix[block_idx] |= 1 << bit_idx;
                        }
                    }
                }
            }
        }

        Self {
            matrix,
            blocks_per_row,
        }
    }

    /// Compare the (u, v) edge with all active edges.
    ///
    /// Returns the amount of active edges found.
    #[inline]
    pub fn count_crossings(&self, u: Idx, v: Idx, active_mask: &[u64]) -> u32 {
        let eid = edge_id(u, v);
        let start = eid * self.blocks_per_row;
        let end = start + self.blocks_per_row;
        let colision_row = &self.matrix[start..end];

        assert_eq!(colision_row.len(), active_mask.len());

        let mut vec_acc = u64x4::splat(0);
        let mut idx = 0;

        while idx + 4 <= colision_row.len() {
            let c = u64x4::from_slice(&colision_row[idx..idx + 4]);
            let a = u64x4::from_slice(&active_mask[idx..idx + 4]);

            vec_acc += (c & a).count_ones();

            idx += 4;
        }

        let mut acc = vec_acc.reduce_sum() as u32;

        while idx < colision_row.len() {
            acc += (colision_row[idx] & active_mask[idx]).count_ones();
            idx += 1;
        }

        acc
    }

    /// Compare the (u, v) edge with all active edges.
    ///
    /// Returns if there is any active edge found.
    ///
    ///
    /// * Complexity: `O(V²)`, but in pratice it is something closer to `O(V²/256)`. Since the
    /// > node count is low (the bigger instance has 120 nodes), this divisor is important in pratice.
    ///
    /// The length of `active_mask` is proportional to the total number of possible edges `E ≈ V²`.
    /// However, by using bitwise operations and `u64x4` SIMD, it processes 256 potential
    /// collisions per loop iteration, acting as a massive constant speedup.
    #[inline]
    pub fn any_crossings(&self, source: Idx, target: Idx, active_mask: &[u64]) -> bool {
        let eid = edge_id(source, target);
        let start = eid * self.blocks_per_row;
        let end = start + self.blocks_per_row;
        let colision_row = &self.matrix[start..end];

        debug_assert_eq!(colision_row.len(), active_mask.len());

        let mut i = 0;
        while i + 8 <= colision_row.len() {
            let c1 = u64x4::from_slice(&colision_row[i..i + 4]);
            let a1 = u64x4::from_slice(&active_mask[i..i + 4]);
            let c2 = u64x4::from_slice(&colision_row[i + 4..i + 8]);
            let a2 = u64x4::from_slice(&active_mask[i + 4..i + 8]);

            let combined = (c1 & a1) | (c2 & a2);
            if combined.reduce_or() != 0 {
                return true;
            }
            i += 8;
        }

        while i < colision_row.len() {
            if (colision_row[i] & active_mask[i]) != 0 {
                return true;
            }
            i += 1;
        }

        false
    }
}
