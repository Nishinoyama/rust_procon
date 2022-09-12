use crate::algebra::{Magma, Monoid};
use crate::structure::ranged::{LeftFixedOp, RangeOp};
use std::ops::Range;

pub struct NaiveVec<T> {
    data: Vec<T>,
}

impl<T: Magma> NaiveVec<T> {
    pub fn build_with(a: &[T]) -> Self {
        Self { data: a.to_vec() }
    }
}

impl<T: Monoid> LeftFixedOp<T> for NaiveVec<T> {
    fn right_op(&mut self, r: usize) -> T {
        self.range_op(0..r)
    }
}

impl<T: Monoid> RangeOp<T> for NaiveVec<T> {
    fn range_op(&mut self, range: Range<usize>) -> T {
        self.data[range].iter().fold(T::id(), |a, x| a.op(x))
    }
}

impl<T: Magma> From<Vec<T>> for NaiveVec<T> {
    fn from(data: Vec<T>) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::typical::MinMonoid;
    use crate::algebra::{Magma, Monoid};
    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::RangeOp;

    #[test]
    fn sparse_min() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let x = x.into_iter().map(MinMonoid).collect::<Vec<_>>();
        let mut nv = NaiveVec::from(x.clone());
        for i in 0..=x.len() {
            for j in i..=x.len() {
                let naive = x[i..j].iter().fold(MinMonoid::id(), |acc, x| acc.op(x));
                assert_eq!(nv.range_op(i..j), naive);
            }
        }
    }
}
