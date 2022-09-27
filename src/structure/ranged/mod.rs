#[allow(unused_imports)]
use crate::algebra::*;
use std::ops::Range;

/// Build: O(N), [LeftFixedFold]: O(1), needs [Monoid].
///
/// For [Group], [RangeFold]: O(1) is valid.
pub mod accumulative_array;
/// Build: O(N), [LeftFixedFold]: O(logN), needs [Monoid].
///
/// For [Commutativity], [PointOpAssign]: O(logN) is valid.
///
/// For [Group], [RangeFold]: O(logN) is valid.
pub mod fenwick_tree;
/// Build: O(N), [RangeFold]: O(N), needs [Magma]. Used for the other structures verification.
pub mod naive_vec;
/// Build: O(N), [RangeFold]: O(logN), needs [Monoid].
/// [PointAssign]: O(logN) is valid.
pub mod segment_tree;
/// Build: O(NlogN), [RangeFold]: O(1), needs [Idempotence], [Monoid].
pub mod sparse_table;

/// Able to assign a_i into elem.
pub trait PointAssign<E, T> {
    fn set_at(&mut self, elem: E, index: usize);
}

/// Returns `OP i \in [l,r) a_i`. If l = r, then returns identity.
pub trait RangeFold<E, T> {
    /// returns `OP i \in [l,r) a_i`.
    /// `self` is mutable since some data structure needs that(such as lazy evaluation).
    fn fold_in(&mut self, range: Range<usize>) -> E;
}

/// Returns `OP i \in [0,r) a_i`. If r = 0, then returns identity.
pub trait LeftFixedFold<E, T> {
    /// returns `OP i \in [0,r) a_i`.
    /// `self` is mutable since some data structure needs that(such as lazy evaluation).
    fn fold_to(&mut self, r: usize) -> E;
}
