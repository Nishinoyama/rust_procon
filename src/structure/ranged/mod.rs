#[allow(unused_imports)]
use crate::algebra::*;
use std::ops::Range;

/// build: O(N), range-op: O(N), needs [Magma].
pub mod naive_vec;
/// build: O(NlogN), range-op: O(1), needs [IdempotentOp], [Monoid].
pub mod sparse_table;

/// returns `OP i \in [l,r) a_i`.
pub trait RangeOp<T> {
    /// returns `OP i \in [l,r) a_i`.
    /// `self` is mutable since some data structure needs that(such as lazy evaluation).
    fn range_op(&mut self, range: Range<usize>) -> T;
}

/// returns `OP i \in [0,r) a_i`.
pub trait LeftFixedOp<T> {
    /// returns `OP i \in [0,r) a_i`.
    /// `self` is mutable since some data structure needs that(such as lazy evaluation).
    fn right_op(&mut self, r: usize) -> T;
}
