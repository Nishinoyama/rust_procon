use crate::algebra::{Commutativity, Group, Monoid};
use crate::structure::ranged::{LeftFixedOp, RangeOp};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct AccumulativeArray<T> {
    data: Vec<T>,
}

impl<T: Monoid> AccumulativeArray<T> {
    pub fn build_with(a: &[T]) -> Self {
        let mut data = std::iter::repeat(T::id())
            .take(a.len() + 1)
            .collect::<Vec<_>>();
        for (i, x) in a.iter().enumerate() {
            data[i + 1] = data[i].op(x)
        }
        Self { data }
    }
}

impl<T: Monoid> LeftFixedOp<T> for AccumulativeArray<T> {
    fn right_op(&mut self, r: usize) -> T {
        self.data[r].clone()
    }
}

impl<T: Group + Commutativity> RangeOp<T> for AccumulativeArray<T> {
    fn range_op(&mut self, range: Range<usize>) -> T {
        self.data[range.end].op(&self.data[range.start].inv())
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
        let x = x.into_iter().map(AdditiveStruct).collect::<Vec<_>>();
        let mut nv = NaiveVec::from(x.clone());
        let mut ac = AccumulativeArray::build_with(&x);
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
