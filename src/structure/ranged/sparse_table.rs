use std::marker::PhantomData;
use crate::algebra::{Commutativity, Idempotence, Monoid};
use crate::structure::ranged::{BuildableWithSlice, RangeOp};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct SparseTable<E, T> {
    alg: PhantomData<T>,
    doubling: Vec<Vec<E>>,
}

impl<E: Clone, T: Monoid<E>> BuildableWithSlice<E, T> for SparseTable<E, T> {
    fn build_with(a: &[E]) -> Self {
        let n = a.len();
        let t = (n as f32).log2() as usize;
        let mut doubling = a.iter().map(|x| vec![x.clone()]).collect::<Vec<_>>();
        for j in 0..t {
            for i in 0..n {
                let db = T::op(&doubling[i][j], &doubling[(n - 1).min(i + (1 << j))][j]);
                doubling[i].push(db);
            }
        }
        Self { alg: Default::default(), doubling }
    }
}

impl<E, T> RangeOp<E, T> for SparseTable<E, T>
    where T: Monoid<E> + Idempotence<E> + Commutativity<E>
{
    fn range_op(&mut self, range: Range<usize>) -> E {
        assert!(range.end <= self.doubling.len());
        let c = range.len();
        if c == 0 {
            T::id()
        } else {
            let t = (c as f32).log2() as usize;
            let d = 1 << t;

            T::op(&self.doubling[range.start][t], &self.doubling[range.end - d][t])
        }
    }
}

#[cfg(test)]
mod test {
    use super::SparseTable;
    use crate::algebra::typical::{MaxMonoid};
    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::{BuildableWithSlice, RangeOp};

    #[test]
    fn sparse_min() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut st = SparseTable::<i32, MaxMonoid>::build_with(&x);
        let mut nv = NaiveVec::<i32, MaxMonoid>::from(x.clone());
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(st.range_op(i..j), nv.range_op(i..j));
            }
        }
    }
}
