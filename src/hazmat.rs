//! Components to build your own primality test.
//! Handle with care.

mod gcd;
mod jacobi;
mod lucas;
mod miller_rabin;
mod precomputed;
#[cfg(test)]
pub(crate) mod primes;
#[cfg(test)]
pub(crate) mod pseudoprimes;
mod sieve;

pub use lucas::{lucas_test, AStarBase, BruteForceBase, LucasBase, LucasCheck, SelfridgeBase};
pub use miller_rabin::MillerRabin;
pub use sieve::{random_odd_integer, SetBits, SmallPrimesSieve, SmallPrimesSieveFactory};

/// Possible results of various primality tests.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Primality {
    /// The number is definitely prime.
    Prime,
    /// The number is probably prime
    /// (see the documentation for the details about possible false positives).
    ProbablyPrime,
    /// The number is definitely composite.
    Composite,
}

impl Primality {
    /// Returns `true` if the result indicates that the number is probably or definitely prime.
    pub fn is_probably_prime(&self) -> bool {
        match self {
            Self::Prime => true,
            Self::ProbablyPrime => true,
            Self::Composite => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::Primality;

    #[test]
    fn primality_derived_traits() {
        assert_eq!(format!("{:?}", Primality::Prime), "Prime");
        assert_eq!(Primality::Prime, Primality::Prime);
        assert!(Primality::Prime != Primality::ProbablyPrime);
        assert_eq!(Primality::Prime.clone(), Primality::Prime);
    }

    #[test]
    fn primality_to_bool() {
        assert!(Primality::Prime.is_probably_prime());
        assert!(Primality::ProbablyPrime.is_probably_prime());
        assert!(!Primality::Composite.is_probably_prime());
    }
}
