use crate::algebra::Monoid;
use crate::structure::ranged::{LeftFixedFold, PointAssign, RangeFold};
use std::marker::PhantomData;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct SegmentTree<E, T> {
    alg: PhantomData<T>,
    data: Vec<E>,
}

impl<E: Clone, T: Monoid<E>> SegmentTree<E, T> {
    #[inline]
    fn len(&self) -> usize {
        self.data.len() / 2
    }
}

impl<E: Clone, T: Monoid<E>> PointAssign<E, T> for SegmentTree<E, T> {
    fn set_at(&mut self, elem: E, index: usize) {
        let mut index = index + self.len();
        self.data[index] = elem;
        while index > 1 {
            index /= 2;
            self.data[index] = T::op(&self.data[index * 2], &self.data[index * 2 + 1])
        }
    }
}

impl<E: Clone, T: Monoid<E>> From<Vec<E>> for SegmentTree<E, T> {
    fn from(a: Vec<E>) -> Self {
        let n = 1 << (a.len() as f64).log2().ceil() as usize;
        let mut data = vec![T::id(); n * 2];
        data[n..n + a.len()].clone_from_slice(&a);
        for i in (1..n).rev() {
            data[i] = T::op(&data[i * 2], &data[i * 2 + 1]);
        }
        Self {
            alg: Default::default(),
            data,
        }
    }
}

impl<E: Clone, T: Monoid<E>> From<&[E]> for SegmentTree<E, T> {
    fn from(a: &[E]) -> Self {
        Self::from(a.to_vec())
    }
}

impl<E: Clone, T: Monoid<E>> LeftFixedFold<E, T> for SegmentTree<E, T> {
    fn fold_to(&mut self, r: usize) -> E {
        let mut res = T::id();
        let mut r = r + self.len();
        while r > 0 {
            if r % 2 == 1 {
                r -= 1;
                res = T::op(&self.data[r], &res);
            }
            r /= 2;
        }
        res
    }
}

impl<E: Clone, T: Monoid<E>> RangeFold<E, T> for SegmentTree<E, T> {
    fn fold_in(&mut self, range: Range<usize>) -> E {
        let mut res_left = T::id();
        let mut res_right = T::id();
        let mut l = range.start + self.len();
        let mut r = range.end + self.len();
        while l < r {
            if l % 2 == 1 {
                res_left = T::op(&res_left, &self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res_right = T::op(&self.data[r], &res_right);
            }
            l /= 2;
            r /= 2;
        }
        T::op(&res_left, &res_right)
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::typical::{MaxMonoid, StringChain};

    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::segment_tree::SegmentTree;
    use crate::structure::ranged::{LeftFixedFold, PointAssign, RangeFold};

    #[test]
    fn seg_max() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut nv = NaiveVec::<i32, MaxMonoid>::from(x.clone());
        let mut st = SegmentTree::<i32, MaxMonoid>::from(x.clone());
        for i in 0..=x.len() {
            assert_eq!(st.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(st.fold_in(i..j), nv.fold_in(i..j));
            }
        }
        for (i, x) in vec![2, 7, 1, 8, 2, 8].into_iter().enumerate() {
            nv.set_at(x, i);
            st.set_at(x, i);
        }
        for i in 0..=x.len() {
            assert_eq!(st.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(st.fold_in(i..j), nv.fold_in(i..j));
            }
        }
    }

    #[test]
    fn seg_string_chain() {
        let x = vec![
            "wow",
            "that",
            "is",
            "mississippi",
            "where",
            "alligators",
            "are",
            "glowing",
            "and",
            "glowing",
            "",
            "!",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
        let mut nv = NaiveVec::<String, StringChain>::from(x.as_slice());
        let mut st = SegmentTree::<String, StringChain>::from(x.as_slice());
        for i in 0..=x.len() {
            assert_eq!(st.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(st.fold_in(i..j), nv.fold_in(i..j));
            }
        }
        for (i, x) in vec!["what", "the", "f@dk", "is", "this"]
            .into_iter()
            .map(String::from)
            .enumerate()
        {
            nv.set_at(x.clone(), i);
            st.set_at(x.clone(), i);
        }
        for i in 0..=x.len() {
            assert_eq!(st.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(st.fold_in(i..j), nv.fold_in(i..j));
            }
        }
    }
}
