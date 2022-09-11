use crate::algebra::{IdempotentOp, Monoid};

struct SparseTree<T> {
    doubling: Vec<Vec<T>>,
}

impl<T: Monoid + IdempotentOp> SparseTree<T> {
    pub fn build_with(a: &[T]) -> Self {
        let n = a.len();
        let t = (n as f32).log2() as usize;
        let mut doubling = a.into_iter().map(|x| vec![x.clone()]).collect::<Vec<_>>();
        for j in 0..t {
            for i in 0..n {
                let db = doubling[i][j].op(&doubling[(n - 1).min(i + (1 << j))][j]);
                doubling[i].push(db);
            }
        }
        Self { doubling }
    }
    /// return `a[l] op a[l+1] op ... op a[r]`
    /// ## assertion
    /// `l < r, r <= len`
    pub fn range_op(&self, l: usize, r: usize) -> T {
        assert!(l <= r);
        assert!(r <= self.doubling.len());
        let c = r - l;
        if c == 0 {
            T::id()
        } else {
            let t = ((r - l) as f32).log2() as usize;
            let d = 1 << t;

            self.doubling[l][t].op(&self.doubling[r - d][t])
        }
    }
}

#[cfg(test)]
mod test {
    use super::SparseTree;
    use crate::algebra::typical::MinMonoid;
    use crate::algebra::{Magma, Monoid};

    #[test]
    fn sparse_min() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let x = x.into_iter().map(|x| MinMonoid(x)).collect::<Vec<_>>();
        let st = SparseTree::build_with(&x);
        for i in 0..x.len() {
            for j in i..x.len() {
                let naive = x[i..j]
                    .into_iter()
                    .fold(MinMonoid::id(), |acc, x| acc.op(x));
                assert_eq!(st.range_op(i, j), naive);
            }
        }
    }
}
