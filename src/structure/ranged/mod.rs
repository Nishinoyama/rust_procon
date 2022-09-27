#[allow(unused_imports)]
use crate::algebra::*;
use std::ops::Range;

/// Build: O(N), [LeftFixedOp]: O(1), needs [Monoid].
///
/// For [Group], [RangeOp]: O(1) is valid.
pub mod accumulative_array;
/// Build: O(N), [LeftFixedOp]: O(logN), needs [Monoid].
///
/// For [Commutativity], [PointOpAssign]: O(logN) is valid.
///
/// For [Group], [RangeOp]: O(logN) is valid.
pub mod fenwick_tree;
/// Build: O(N), [RangeOp]: O(N), needs [Magma]. Used for the other structures verification.
pub mod naive_vec;
/// Build: O(NlogN), [RangeOp]: O(1), needs [Idempotence], [Monoid].
pub mod sparse_table;

/// returns `OP i \in [l,r) a_i`. If l < r, then returns identity.
pub trait RangeOp<E, T: Monoid<E>> {
    /// returns `OP i \in [l,r) a_i`.
    /// `self` is mutable since some data structure needs that(such as lazy evaluation).
    fn range_op(&mut self, range: Range<usize>) -> E;
}

/// returns `OP i \in [0,r) a_i`. If r = 0, then returns identity.
pub trait LeftFixedOp<E, T> {
    /// returns `OP i \in [0,r) a_i`.
    /// `self` is mutable since some data structure needs that(such as lazy evaluation).
    fn right_op(&mut self, r: usize) -> E;
}

///
pub trait BuildableWithSlice<E, T> {
    fn build_with(a: &[E]) -> Self;
}
