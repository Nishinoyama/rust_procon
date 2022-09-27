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
/// Build: O(N), [RangeOp]: O(logN), needs [Monoid].
/// [PointAssign]: O(logN) is valid.
pub mod segment_tree;
/// Build: O(NlogN), [RangeOp]: O(1), needs [Idempotence], [Monoid].
pub mod sparse_table;

/// Able to assign a_i into elem.
pub trait PointAssign<E, T> {
    fn set_at(&mut self, elem: E, index: usize);
}

/// Returns `OP i \in [l,r) a_i`. If l = r, then returns identity.
pub trait RangeOp<E, T> {
    /// returns `OP i \in [l,r) a_i`.
    /// `self` is mutable since some data structure needs that(such as lazy evaluation).
    fn range_op(&mut self, range: Range<usize>) -> E;
}

/// Returns `OP i \in [0,r) a_i`. If r = 0, then returns identity.
pub trait LeftFixedOp<E, T> {
    /// returns `OP i \in [0,r) a_i`.
    /// `self` is mutable since some data structure needs that(such as lazy evaluation).
    fn right_op(&mut self, r: usize) -> E;
}

/// Able to build a Range-Featured Structure with [slice]
#[deprecated(note = "use std::convert::From instead")]
pub trait BuildableWithSlice<E, T> {
    /// Build a structure. Complexity is depends on each structure.
    fn build_with(a: &[E]) -> Self;
}
