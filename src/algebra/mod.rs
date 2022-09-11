use num_traits::Bounded;

/// Those op is **closed**.
pub trait Magma: Clone {
    fn op(&self, other: &Self) -> Self;
}

/// Those op is **associative**.
///
/// ## Associative
/// for all a, b, c, (a `op` b) `op` c = a `op` (b `op` c)
pub trait Semigroup: Magma {}

/// Those op is **associative** + an **identity element** is existed.
///
/// ## Identity
/// there exists e, for all a, a `op` e = e `op` a = a
/// such element is written as e
pub trait Monoid: Semigroup {
    fn id() -> Self;
}

/// Those op is **associative** + an **identity element** is existed +
/// every element has an **inverse element**.
///
/// ## Invertibility
/// for all a, there exists b, a `op` b = b `op` a = e,
/// where e is identity element.
///
pub trait Group: Monoid {
    fn inv(&self) -> Self;
}

/// Those op is **idempotent**.
///
/// ## Idempotence
/// for all a, a `op` a = a
///
/// ## Example
/// max: `max x x = x`
///
/// gcd: `gcd x x = x`
pub trait IdempotentOp: Magma {}

/// Frequently used algebraic structures.
pub mod typical {
    use super::*;

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

    /// min: [Monoid] and [IdempotentOp]
    #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
    pub struct MinMonoid<T: Clone + Ord>(pub T);

    impl<T: Clone + Ord> Semigroup for MinMonoid<T> {}

    impl<T: Clone + Ord> Magma for MinMonoid<T> {
        fn op(&self, other: &Self) -> Self {
            self.min(&other).clone()
        }
    }

    impl<T: Clone + Ord + Bounded> Monoid for MinMonoid<T> {
        fn id() -> Self {
            Self(T::max_value())
        }
    }

    impl<T: Clone + Ord> IdempotentOp for MinMonoid<T> {}

    /// max: [Monoid] and [IdempotentOp]
    #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
    pub struct MaxMonoid<T>(T);

    impl<T: Clone + Ord> Semigroup for MaxMonoid<T> {}

    impl<T: Clone + Ord> Magma for MaxMonoid<T> {
        fn op(&self, other: &Self) -> Self {
            self.max(&other).clone()
        }
    }

    impl<T: Clone + Ord + Bounded> Monoid for MaxMonoid<T> {
        fn id() -> Self {
            Self(T::min_value())
        }
    }

    impl<T: Clone + Ord> IdempotentOp for MaxMonoid<T> {}

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_monoid_max() {
            let (l, r) = (MaxMonoid(0), MaxMonoid(3));
            assert_eq!(l.op(&r), MaxMonoid(3));
            assert_eq!(l.op(&MaxMonoid::id()), l);

            let t = (0..).map(|i| MaxMonoid(i)).take(20000).collect::<Vec<_>>();
            assert_eq!(
                t[20..=40].iter().fold(MaxMonoid::id(), |ac, x| ac.op(x)),
                MaxMonoid(40),
            )
        }

        #[test]
        fn test_monoid_min() {
            let t = MinMonoid(BoundedVec(vec![3, 1, 4]));

            assert_eq!(t.op(&t), t);
            assert_eq!(
                t.op(&MinMonoid(BoundedVec(vec![]))),
                MinMonoid(BoundedVec(vec![]))
            );
        }
    }
}