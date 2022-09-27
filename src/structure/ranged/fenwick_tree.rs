use std::marker::PhantomData;
use crate::algebra::{Commutativity, Group, Monoid};
use crate::structure::ranged::{BuildableWithSlice, LeftFixedOp, RangeOp};
use std::ops::Range;

#[derive(Debug, Clone)]
/// Yee, there is no advantage that use this with Non-[Commutativity] structure.
/// Use [AccumulativeArray](crate::structure::ranged::accumulative_array::AccumulativeArray) in those cases.
pub struct FenwickTree<E, T> {
    alg: PhantomData<T>,
    data: Vec<E>,
}

impl<E: Clone, T: Monoid<E>> FenwickTree<E, T> {
    pub fn new(n: usize) -> Self {
        Self {
            alg: Default::default(),
            data: std::iter::repeat(T::id()).take(n + 1).collect(),
        }
    }
}

impl<E: Clone, T: Monoid<E>> BuildableWithSlice<E, T> for FenwickTree<E, T> {
    fn build_with(a: &[E]) -> Self {
        let mut data = std::iter::repeat(T::id())
            .take(a.len() + 1)
            .collect::<Vec<_>>();
        for (i, x) in a.iter().enumerate() {
            let i = i + 1;
            data[i] = T::op(&data[i], x);
            let j = i + (i & i.wrapping_neg());
            let origin = data[i].clone();
            if let Some(upper) = data.get_mut(j) {
                *upper = T::op(upper, &origin);
            }
        }
        Self { alg: Default::default(), data }
    }
}

impl<E, T: Commutativity<E>> FenwickTree<E, T> {
    pub fn point_op_assign(&mut self, index: usize, rhs: &E) {
        let mut index = index + 1;
        while index < self.data.len() {
            self.data[index] = T::op(&self.data[index], rhs);
            index += index & index.wrapping_neg();
        }
    }
}

impl<E, T: Monoid<E> + Commutativity<E>> FenwickTree<E, T> {}

impl<E, T: Monoid<E>> LeftFixedOp<E, T> for FenwickTree<E, T> {
    fn right_op(&mut self, r: usize) -> E {
        let mut res = T::id();
        let mut r = r;
        while r > 0 {
            res = T::op(&res, &self.data[r]);
            r -= r & r.wrapping_neg();
        }
        res
    }
}

impl<E, T> RangeOp<E, T> for FenwickTree<E, T>
    where T: Group<E> + Commutativity<E>
{
    fn range_op(&mut self, range: Range<usize>) -> E {
        let r = self.right_op(range.end);
        let l = self.right_op(range.start);
        T::op(&r, &T::inv(&l))
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::typical::AdditiveStruct;

    use crate::structure::ranged::fenwick_tree::FenwickTree;
    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::{BuildableWithSlice, LeftFixedOp, RangeOp};

    #[test]
    fn fenwick_sum() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut nv = NaiveVec::<i32, AdditiveStruct>::from(x.clone());
        let mut ft = FenwickTree::<i32, AdditiveStruct>::build_with(&x);
        for i in 0..=x.len() {
            assert_eq!(ft.right_op(i), nv.right_op(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(ft.range_op(i..j), nv.range_op(i..j));
            }
        }
        for (i, x) in vec![2, 7, 1, 8, 2, 8]
            .into_iter()
            .enumerate()
        {
            nv.point_op_assign(i, &x);
            ft.point_op_assign(i, &x);
        }
        for i in 0..=x.len() {
            assert_eq!(ft.right_op(i), nv.right_op(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(ft.range_op(i..j), nv.range_op(i..j));
            }
        }
    }
}
