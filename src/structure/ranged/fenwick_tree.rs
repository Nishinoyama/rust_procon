use crate::algebra::{Commutativity, Group, Monoid};
use crate::structure::ranged::{LeftFixedFold, PointAssign, RangeFold};
use std::marker::PhantomData;
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
impl<E: Clone, T: Monoid<E>> From<Vec<E>> for FenwickTree<E, T> {
    fn from(a: Vec<E>) -> Self {
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
        Self {
            alg: Default::default(),
            data,
        }
    }
}

impl<E: Clone, T: Monoid<E>> From<&[E]> for FenwickTree<E, T> {
    fn from(a: &[E]) -> Self {
        Self::from(a.to_vec())
    }
}

impl<E: Clone, T: Commutativity<E> + Group<E>> PointAssign<E, T> for FenwickTree<E, T> {
    fn set_at(&mut self, elem: E, index: usize) {
        let inv = T::inv(&self.fold_in(index..index + 1));
        self.point_op_assign(index, &T::inv(&inv));
        self.point_op_assign(index, &elem);
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

impl<E, T: Monoid<E>> LeftFixedFold<E, T> for FenwickTree<E, T> {
    fn fold_to(&mut self, r: usize) -> E {
        let mut res = T::id();
        let mut r = r;
        while r > 0 {
            res = T::op(&res, &self.data[r]);
            r -= r & r.wrapping_neg();
        }
        res
    }
}

impl<E, T> RangeFold<E, T> for FenwickTree<E, T>
where
    T: Group<E> + Commutativity<E>,
{
    fn fold_in(&mut self, range: Range<usize>) -> E {
        let r = self.fold_to(range.end);
        let l = self.fold_to(range.start);
        T::op(&r, &T::inv(&l))
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::typical::AdditiveStruct;

    use crate::structure::ranged::fenwick_tree::FenwickTree;
    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::{LeftFixedFold, RangeFold};

    #[test]
    fn fenwick_sum() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut nv = NaiveVec::<i32, AdditiveStruct>::from(x.clone());
        let mut ft = FenwickTree::<i32, AdditiveStruct>::from(x.clone());
        for i in 0..=x.len() {
            assert_eq!(ft.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(ft.fold_in(i..j), nv.fold_in(i..j));
            }
        }
        for (i, x) in vec![2, 7, 1, 8, 2, 8].into_iter().enumerate() {
            nv.point_op_assign(i, &x);
            ft.point_op_assign(i, &x);
        }
        for i in 0..=x.len() {
            assert_eq!(ft.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(ft.fold_in(i..j), nv.fold_in(i..j));
            }
        }
    }
}
