use std::marker::PhantomData;
use crate::algebra::{Commutativity, Group, Monoid};
use crate::structure::ranged::{LeftFixedOp, RangeOp};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct AccumulativeArray<E, T> {
    alg: PhantomData<T>,
    data: Vec<E>,
}

impl<E: Clone, T: Monoid<E>> AccumulativeArray<E, T> {
    pub fn build_with(a: &[E]) -> Self {
        let mut data = std::iter::repeat(T::id())
            .take(a.len() + 1)
            .collect::<Vec<_>>();
        for (i, x) in a.iter().enumerate() {
            data[i + 1] = T::op(&data[i], x)
        }
        Self { alg: Default::default(), data }
    }
}

impl<E: Clone, T: Monoid<E>> LeftFixedOp<E, T> for AccumulativeArray<E, T> {
    fn right_op(&mut self, r: usize) -> E {
        self.data[r].clone()
    }
}

impl<T: Group<E> + Commutativity<E>, E> RangeOp<E, T> for AccumulativeArray<E, T> {
    fn range_op(&mut self, range: Range<usize>) -> E {
        T::op(&self.data[range.end], &T::inv(&self.data[range.start]))
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::typical::AdditiveStruct;

    use crate::structure::ranged::accumulative_array::AccumulativeArray;
    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::{LeftFixedOp, RangeOp};

    #[test]
    fn acc_sum() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut nv = NaiveVec::<i32, AdditiveStruct>::from(x.clone());
        let mut ac = AccumulativeArray::<i32, AdditiveStruct>::build_with(&x);
        for i in 0..=x.len() {
            assert_eq!(ac.right_op(i), nv.right_op(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(ac.range_op(i..j), nv.range_op(i..j));
            }
        }
    }
}
