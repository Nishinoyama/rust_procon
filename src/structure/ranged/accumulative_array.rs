use crate::algebra::{Commutativity, Group, Monoid};
use crate::structure::ranged::{LeftFixedFold, RangeFold};
use std::marker::PhantomData;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct AccumulativeArray<E, T> {
    alg: PhantomData<T>,
    data: Vec<E>,
}

impl<E: Clone, T: Monoid<E>> From<Vec<E>> for AccumulativeArray<E, T> {
    fn from(a: Vec<E>) -> Self {
        let mut data = std::iter::repeat(T::id())
            .take(a.len() + 1)
            .collect::<Vec<_>>();
        for (i, x) in a.iter().enumerate() {
            data[i + 1] = T::op(&data[i], x)
        }
        Self {
            alg: Default::default(),
            data,
        }
    }
}

impl<E: Clone, T: Monoid<E>> From<&[E]> for AccumulativeArray<E, T> {
    fn from(a: &[E]) -> Self {
        Self::from(a.to_vec())
    }
}

impl<E: Clone, T: Monoid<E>> LeftFixedFold<E, T> for AccumulativeArray<E, T> {
    fn fold_to(&mut self, r: usize) -> E {
        self.data[r].clone()
    }
}

impl<E, T> RangeFold<E, T> for AccumulativeArray<E, T>
where
    T: Group<E> + Commutativity<E>,
{
    fn fold_in(&mut self, range: Range<usize>) -> E {
        T::op(&self.data[range.end], &T::inv(&self.data[range.start]))
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::typical::AdditiveStruct;

    use crate::structure::ranged::accumulative_array::AccumulativeArray;
    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::{LeftFixedFold, RangeFold};

    #[test]
    fn acc_sum() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut nv = NaiveVec::<i32, AdditiveStruct>::from(x.clone());
        let mut ac = AccumulativeArray::<i32, AdditiveStruct>::from(x.clone());
        for i in 0..=x.len() {
            assert_eq!(ac.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(ac.fold_in(i..j), nv.fold_in(i..j));
            }
        }
    }
}
