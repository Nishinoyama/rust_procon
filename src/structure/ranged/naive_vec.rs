use crate::algebra::{Magma, Monoid};
use crate::structure::ranged::{LeftFixedFold, PointAssign, RangeFold};
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

impl<E, T: Monoid<E>> LeftFixedFold<E, T> for NaiveVec<E, T> {
    fn fold_to(&mut self, r: usize) -> E {
        self.fold_in(0..r)
    }
}

impl<E, T: Monoid<E>> RangeFold<E, T> for NaiveVec<E, T> {
    fn fold_in(&mut self, range: Range<usize>) -> E {
        self.data[range].iter().fold(T::id(), |a, x| T::op(&a, x))
    }
}

impl<E, T> From<Vec<E>> for NaiveVec<E, T> {
    fn from(data: Vec<E>) -> Self {
        Self {
            alg: Default::default(),
            data,
        }
    }
}

impl<E: Clone, T> From<&[E]> for NaiveVec<E, T> {
    fn from(data: &[E]) -> Self {
        Self::from(data.to_vec())
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::typical::MinMonoid;
    use crate::algebra::{Magma, Monoid};
    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::RangeFold;

    #[test]
    fn sparse_min() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut nv = NaiveVec::<i32, MinMonoid>::from(x.clone());
        for i in 0..=x.len() {
            for j in i..=x.len() {
                let naive = x[i..j]
                    .iter()
                    .fold(MinMonoid::id(), |acc, x| MinMonoid::op(&acc, x));
                assert_eq!(nv.fold_in(i..j), naive);
            }
        }
    }
}
