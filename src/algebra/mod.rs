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

/// Those op is **commutative**.
///
/// ## Commutativity
/// for all a b, a `op` b = b `op` a
///
pub trait Commutativity: Magma {}

/// Those op is **idempotent**.
///
/// ## Idempotence
/// for all a, a `op` a = a
///
/// ## Example
/// max: `max x x = x`
///
/// gcd: `gcd x x = x`
pub trait Idempotence: Magma {}

/// Frequently used algebraic structures.
pub mod typical;
