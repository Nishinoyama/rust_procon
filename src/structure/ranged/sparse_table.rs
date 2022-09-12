use crate::algebra::{Idempotence, Monoid};
use crate::structure::ranged::RangeOp;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct SparseTree<T> {
    doubling: Vec<Vec<T>>,
}

impl<T: Monoid + Idempotence> SparseTree<T> {
    pub fn build_with(a: &[T]) -> Self {
        let n = a.len();
        let t = (n as f32).log2() as usize;
        let mut doubling = a.iter().map(|x| vec![x.clone()]).collect::<Vec<_>>();
        for j in 0..t {
            for i in 0..n {
                let db = doubling[i][j].op(&doubling[(n - 1).min(i + (1 << j))][j]);
                doubling[i].push(db);
            }
        }
        Self { doubling }
    }
}

impl<T: Monoid + Idempotence> RangeOp<T> for SparseTree<T> {
    fn range_op(&mut self, range: Range<usize>) -> T {
        assert!(range.end <= self.doubling.len());
        let c = range.len();
        if c == 0 {
            T::id()
        } else {
            let t = (c as f32).log2() as usize;
            let d = 1 << t;

            self.doubling[range.start][t].op(&self.doubling[range.end - d][t])
        }
    }
}

#[cfg(test)]
mod test {
    use super::SparseTree;
    use crate::algebra::typical::MinMonoid;
    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::RangeOp;

    #[test]
    fn sparse_min() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let x = x.into_iter().map(MinMonoid).collect::<Vec<_>>();
        let mut st = SparseTree::build_with(&x);
        let mut nv = NaiveVec::from(x.clone());
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(st.range_op(i..j), nv.range_op(i..j));
            }
        }
    }
}
