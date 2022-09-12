use crate::algebra::{Commutativity, Group, Monoid};
use crate::structure::ranged::{LeftFixedOp, RangeOp};
use std::ops::Range;

#[derive(Debug, Clone)]
/// Yee, there is no advantage that use this with Non-[Commutativity] structure.
/// Use [AccumulativeArray](crate::structure::ranged::accumulative_array::AccumulativeArray) in those cases.
pub struct FenwickTree<T> {
    data: Vec<T>,
}

impl<T: Monoid> FenwickTree<T> {
    pub fn new(n: usize) -> Self {
        Self {
            data: std::iter::repeat(T::id()).take(n + 1).collect(),
        }
    }
    pub fn build_with(a: &[T]) -> Self {
        let mut data = std::iter::repeat(T::id())
            .take(a.len() + 1)
            .collect::<Vec<_>>();
        for (i, x) in a.iter().enumerate() {
            let i = i + 1;
            data[i] = data[i].op(x);
            let j = i + (i & i.wrapping_neg());
            let origin = data[i].clone();
            if let Some(upper) = data.get_mut(j) {
                *upper = upper.op(&origin);
            }
        }
        Self { data }
    }
}

impl<T: Commutativity> FenwickTree<T> {
    pub fn point_op_assign(&mut self, index: usize, rhs: &T) {
        let mut index = index + 1;
        while index < self.data.len() {
            self.data[index] = self.data[index].op(rhs);
            index += index & index.wrapping_neg();
        }
    }
}

impl<T: Monoid + Commutativity> FenwickTree<T> {}

impl<T: Monoid> LeftFixedOp<T> for FenwickTree<T> {
    fn right_op(&mut self, r: usize) -> T {
        let mut res = T::id();
        let mut r = r;
        while r > 0 {
            res = res.op(&self.data[r]);
            r -= r & r.wrapping_neg();
        }
        res
    }
}

impl<T: Group + Commutativity> RangeOp<T> for FenwickTree<T> {
    fn range_op(&mut self, range: Range<usize>) -> T {
        let r = self.right_op(range.end);
        let l = self.right_op(range.start);
        r.op(&l.inv())
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::typical::AdditiveStruct;

    use crate::structure::ranged::fenwick_tree::FenwickTree;
    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::{LeftFixedOp, RangeOp};

    #[test]
    fn fenwick_sum() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let x = x.into_iter().map(AdditiveStruct).collect::<Vec<_>>();
        let mut nv = NaiveVec::from(x.clone());
        let mut ft = FenwickTree::build_with(&x);
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
            .map(AdditiveStruct)
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
