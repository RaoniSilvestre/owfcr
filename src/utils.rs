//! Utilities to the other parts of the code.
//!
//! Mainly focusing on edge identification and utilities to find out how many u64 we need to store
//! all the intersection information about a single edge.

use crate::types::{Graph, Idx};

/// Pairing function N² -> N.
///
/// For this function to work, `u` must be different of `v`.
///
/// If that is the case, the function will take a pair from N² and maps it directly into N. Being
/// injective and sobrejective.
#[inline]
pub fn edge_id(u: Idx, v: Idx) -> usize {
    assert_ne!(u, v);

    let (min, max) = if u < v { (u, v) } else { (v, u) };

    (max * (max - 1)) / 2 + min
}

/// Generate the (u, v) pair from a flattened 1D array position.
///
/// It is the inverse operation of `edge_index`.
#[inline]
pub fn idx_to_edge(i: usize, n: usize) -> (Idx, Idx) {
    (i / n, i % n)
}

/// Flattens a (u, v) vertex pair into a one dimension index.
///
/// It is the inverse operation of `get_edge`.
#[inline]
pub fn edge_to_idx(u: Idx, v: Idx, n: usize) -> usize {
    u * n + v
}

/// How many blocks of 64 bits we need to represent one row
pub fn block_size(g: &Graph) -> usize {
    max_edges(g).div_ceil(64)
}

/// How many edges a complete graph will have (which is always the case in WFCR)
pub fn max_edges(g: &Graph) -> usize {
    let n = g.node_count();

    (n * (n - 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_index_inversion() {
        let n = 10; // Matrix size example

        // 1. Test f(g(i)) = i  (From 1D into 2D, and back into 1D)
        for i in 0..(n * n) {
            let (u, v) = idx_to_edge(i, n);
            assert_eq!(edge_to_idx(u, v, n), i, "Failed for index {i}");
        }

        // 2. Test g(f(u,v)) = (u,v) (From 2D into 1D, and back into 2D)
        for u in 0..n {
            for v in 0..n {
                let i = edge_to_idx(u, v, n);
                let (res_u, res_v) = idx_to_edge(i, n);

                assert_eq!(res_u, u);
                assert_eq!(res_v, v);
            }
        }
    }
}
