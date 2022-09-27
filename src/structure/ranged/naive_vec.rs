use crate::algebra::{Magma, Monoid};
use crate::structure::ranged::{LeftFixedOp, PointAssign, RangeOp};
use std::marker::PhantomData;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct NaiveVec<E, T> {
    alg: PhantomData<T>,
    data: Vec<E>,
}

impl<E, T> PointAssign<E, T> for NaiveVec<E, T> {
    fn set_at(&mut self, elem: E, index: usize) {
        self.data[index] = elem
    }
}

impl<E, T: Magma<E>> NaiveVec<E, T> {
    pub fn point_op_assign(&mut self, index: usize, rhs: &E) {
        self.data[index] = T::op(&self.data[index], rhs);
    }
}

impl<E: Clone, T: Magma<E>> NaiveVec<E, T> {
    pub fn build_with(a: &[E]) -> Self {
        Self::from(a)
    }
}

impl<E, T: Monoid<E>> LeftFixedOp<E, T> for NaiveVec<E, T> {
    fn right_op(&mut self, r: usize) -> E {
        self.range_op(0..r)
    }
}

impl<E, T: Monoid<E>> RangeOp<E, T> for NaiveVec<E, T> {
    fn range_op(&mut self, range: Range<usize>) -> E {
        self.data[range].iter().fold(T::id(), |a, x| T::op(&a, x))
    }
}

impl<E: Clone, T> From<&[E]> for NaiveVec<E, T> {
    fn from(data: &[E]) -> Self {
        Self {
            alg: Default::default(),
            data: data.to_vec(),
        }
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
        let mut nv = NaiveVec::<i32, MinMonoid>::from(x.as_slice());
        for i in 0..=x.len() {
            for j in i..=x.len() {
                let naive = x[i..j]
                    .iter()
                    .fold(MinMonoid::id(), |acc, x| MinMonoid::op(&acc, x));
                assert_eq!(nv.range_op(i..j), naive);
            }
        }
    }
}
