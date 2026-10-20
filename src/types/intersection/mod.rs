//! Intersection module.

use crate::types::{Graph, Idx};

mod basic;
mod fast;

#[allow(unused)]
pub use basic::BasicIntersectionTester;
pub use fast::FastIntersectionTester;

/// Verify in the graph if two edges (u1, v1) intersect with (u2, v2).
#[inline]
fn edges_intersect(graph: &Graph, u1: Idx, v1: Idx, u2: Idx, v2: Idx) -> bool {
    if u1 == u2 || u1 == v2 || v1 == u2 || v1 == v2 {
        return false;
    }

    let p1 = graph.node(u1);
    let q1 = graph.node(v1);
    let p2 = graph.node(u2);
    let q2 = graph.node(v2);

    check_geometry((p1.x, p1.y), (q1.x, q1.y), (p2.x, p2.y), (q2.x, q2.y))
}

/// Use the geometry to check if there is any intersection. Found of internet in stack overflow.
///
/// [link](https://stackoverflow.com/questions/26315401/explanation-of-ccw-algorithm)
#[inline]
const fn check_geometry(p1: (f64, f64), q1: (f64, f64), p2: (f64, f64), q2: (f64, f64)) -> bool {
    let a = ccw(p1, p2, q2);
    let b = ccw(q1, p2, q2);
    let c = ccw(p1, q1, p2);
    let d = ccw(p1, q1, q2);

    a != b && c != d
}

/// Use the geometry to check if there is any intersection. Found of internet in stack overflow.
///
/// [link](https://stackoverflow.com/questions/26315401/explanation-of-ccw-algorithm)
#[inline]
const fn ccw(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
    (c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)
}
