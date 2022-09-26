use super::*;
use num_traits::Zero;
use std::ops::{Add, BitXor, Neg};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct BoundedVec<T>(Vec<T>);

impl<T: Bounded> Bounded for BoundedVec<T> {
    fn min_value() -> Self {
        Self(vec![])
    }

    fn max_value() -> Self {
        Self(vec![Bounded::max_value()])
    }
}

// FIXME: implements of monoid or such structure should be written in macro...

/// min: [Monoid] and [Idempotence]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct MinMonoid;

impl<T: Clone + Ord> Magma<T> for MinMonoid {
    fn op(lhs: &T, rhs: &T) -> T {
        lhs.max(rhs).clone()
    }
}

impl<T: Clone + Ord> Semigroup<T> for MinMonoid {}

impl<T: Clone + Ord + Bounded> Monoid<T> for MinMonoid {
    fn id() -> T {
        T::max_value()
    }
}

impl<T: Clone + Ord> Idempotence<T> for MinMonoid {}

impl<T: Clone + Ord> Commutativity<T> for MinMonoid {}

/// max: [Monoid] and [Idempotence]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct MaxMonoid;

impl<T: Clone + Ord> Magma<T> for MaxMonoid {
    fn op(lhs: &T, rhs: &T) -> T {
        lhs.max(rhs).clone()
    }
}

impl<T: Clone + Ord> Semigroup<T> for MaxMonoid {}

impl<T: Clone + Ord + Bounded> Monoid<T> for MaxMonoid {
    fn id() -> T {
        T::min_value()
    }
}

impl<T: Clone + Ord> Idempotence<T> for MaxMonoid {}

impl<T: Clone + Ord> Commutativity<T> for MaxMonoid {}

/// additive: [Monoid] for unsigned, [Group] for singed
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct AdditiveStruct;

impl<T> Magma<T> for AdditiveStruct
where
    T: Clone + Add<Output = T>,
{
    fn op(lhs: &T, rhs: &T) -> T {
        lhs.clone() + rhs.clone()
    }
}

impl<T> Semigroup<T> for AdditiveStruct where T: Clone + Add<Output = T> {}


impl<T> Monoid<T> for AdditiveStruct
where
    T: Clone + Add<Output = T> + Zero,
{
    fn id() -> T {
        T::zero()
    }
}

impl<T> Group<T> for AdditiveStruct
where
    T: Clone + Add<Output = T> + Zero + Neg<Output = T>,
{
    fn inv(elm: &T) -> T {
        -elm.clone()
    }
}

impl<T> Commutativity<T> for AdditiveStruct where T: Clone + Add<Output = T> {}

/// bitwise-xor: [Group]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct BitXorGroup;

impl<T> Semigroup<T> for BitXorGroup where T: Clone + BitXor<Output = T> {}

impl<T> Magma<T> for BitXorGroup
where
    T: Clone + BitXor<Output = T>,
{
    fn op(lhs: &T, rhs: &T) -> T {
        lhs.clone().bitxor(rhs.clone())
    }
}

impl<T> Monoid<T> for BitXorGroup
where
    T: Clone + BitXor<Output = T> + Zero,
{
    fn id() -> T {
        T::zero()
    }
}

impl<T> Group<T> for BitXorGroup
where
    T: Clone + BitXor<Output = T> + Zero,
{
    fn inv(elm: &T) -> T {
        elm.clone()
    }
}

impl<T> Commutativity<T> for BitXorGroup where T: Clone + BitXor<Output = T> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_monoid_max() {
        let (l, r) = (0, 3);
        assert_eq!(MaxMonoid::op(&l, &r), 3);
        assert_eq!(MaxMonoid::op(&l, &MaxMonoid::id()), l);

        assert_eq!(
            (20usize..=40).into_iter().fold(MaxMonoid::id(), |ac, x| MaxMonoid::op(&ac, &x)),
            40,
        )
    }

}
