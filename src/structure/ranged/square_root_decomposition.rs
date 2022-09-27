use crate::algebra::Monoid;
use crate::structure::ranged::{LeftFixedFold, PointAssign, RangeFold};
use std::marker::PhantomData;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct SquareRootDecomposition<E, T> {
    alg: PhantomData<T>,
    data: Vec<E>,
    blocks: Vec<E>,
}

impl<E: Clone, T: Monoid<E>> SquareRootDecomposition<E, T> {
    #[inline]
    fn block_len(&self) -> usize {
        self.blocks.len()
    }
    #[inline]
    fn naive_fold(&self, range: Range<usize>) -> E {
        self.data[range]
            .iter()
            .fold(T::id(), |acc, x| T::op(&acc, x))
    }
}

impl<E: Clone, T: Monoid<E>> From<Vec<E>> for SquareRootDecomposition<E, T> {
    fn from(data: Vec<E>) -> Self {
        let block_len = (1usize..).find(|&i| i * i >= data.len()).unwrap();
        let mut blocks = vec![T::id(); block_len];
        for (i, e) in data.iter().enumerate() {
            blocks[i / block_len] = T::op(&blocks[i / block_len], e);
        }
        Self {
            alg: Default::default(),
            data,
            blocks,
        }
    }
}

impl<E: Clone, T: Monoid<E>> From<&[E]> for SquareRootDecomposition<E, T> {
    fn from(a: &[E]) -> Self {
        Self::from(a.to_vec())
    }
}

impl<E: Clone, T: Monoid<E>> PointAssign<E, T> for SquareRootDecomposition<E, T> {
    fn set_at(&mut self, elem: E, index: usize) {
        let block_i = index / self.block_len();
        self.data[index] = elem;
        self.blocks[block_i] = T::id();
        for x in self.data[block_i * self.block_len()..]
            .iter()
            .take(self.block_len())
        {
            self.blocks[block_i] = T::op(&self.blocks[block_i], x)
        }
    }
}

impl<E: Clone, T: Monoid<E>> LeftFixedFold<E, T> for SquareRootDecomposition<E, T> {
    fn fold_to(&mut self, r: usize) -> E {
        let mut res = T::id();
        for b in 0..self.block_len() {
            let i = b * self.block_len();
            let j = i + self.block_len();
            if j < r {
                res = T::op(&res, &self.blocks[b]);
            } else if i <= r {
                res = T::op(&res, &self.naive_fold(i..r));
            } else {
                break;
            }
        }
        res
    }
}

impl<E: Clone, T: Monoid<E>> RangeFold<E, T> for SquareRootDecomposition<E, T> {
    fn fold_in(&mut self, range: Range<usize>) -> E {
        let mut res = T::id();
        for b in 0..self.block_len() {
            let i = b * self.block_len();
            let j = i + self.block_len();
            if j < range.start || range.end < i {
                continue;
            }
            if range.start <= i && j <= range.end {
                res = T::op(&res, &self.blocks[b]);
            } else if i <= range.end {
                res = T::op(
                    &res,
                    &self.naive_fold((range.start.max(i))..range.end.min(j)),
                );
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::typical::{MaxMonoid, StringChain};

    use crate::structure::ranged::naive_vec::NaiveVec;
    use crate::structure::ranged::square_root_decomposition::SquareRootDecomposition;
    use crate::structure::ranged::{LeftFixedFold, PointAssign, RangeFold};

    #[test]
    fn sqrt_dec_max() {
        let x = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8];
        let mut nv = NaiveVec::<i32, MaxMonoid>::from(x.clone());
        let mut srd = SquareRootDecomposition::<i32, MaxMonoid>::from(x.clone());
        println!("{:?}", srd);
        for i in 0..=x.len() {
            assert_eq!(srd.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(srd.fold_in(i..j), nv.fold_in(i..j));
            }
        }
        for (i, x) in vec![2, 7, 1, 8, 2, 8].into_iter().enumerate() {
            nv.set_at(x, i);
            srd.set_at(x, i);
        }
        for i in 0..=x.len() {
            assert_eq!(srd.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(srd.fold_in(i..j), nv.fold_in(i..j));
            }
        }
    }

    #[test]
    fn sqrt_dec_string_chain() {
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
        let mut srd = SquareRootDecomposition::<String, StringChain>::from(x.as_slice());
        for i in 0..=x.len() {
            assert_eq!(srd.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(srd.fold_in(i..j), nv.fold_in(i..j));
            }
        }
        for (i, x) in vec!["what", "the", "f@dk", "is", "this"]
            .into_iter()
            .map(String::from)
            .enumerate()
        {
            nv.set_at(x.clone(), i);
            srd.set_at(x.clone(), i);
        }
        for i in 0..=x.len() {
            assert_eq!(srd.fold_to(i), nv.fold_to(i));
        }
        for i in 0..=x.len() {
            for j in i..=x.len() {
                assert_eq!(srd.fold_in(i..j), nv.fold_in(i..j));
            }
        }
    }
}
