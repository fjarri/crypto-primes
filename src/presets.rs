use crypto_bigint::{Integer, Uint};
use rand_core::CryptoRngCore;

#[cfg(feature = "default-rng")]
use rand_core::OsRng;

use crate::hazmat::{
    lucas_test, random_odd_uint, LucasCheck, MillerRabin, Primality, SelfridgeBase, Sieve,
};

/// Returns a random prime of size `bit_length` using [`OsRng`] as the RNG.
///
/// See [`is_prime_with_rng`] for details about the performed checks.
#[cfg(feature = "default-rng")]
pub fn prime<const L: usize>(bit_length: usize) -> Uint<L> {
    prime_with_rng(&mut OsRng, bit_length)
}

/// Returns a random safe prime (that is, such that `(n - 1) / 2` is also prime)
/// of size `bit_length` using [`OsRng`] as the RNG.
///
/// See [`is_prime_with_rng`] for details about the performed checks.
#[cfg(feature = "default-rng")]
pub fn safe_prime<const L: usize>(bit_length: usize) -> Uint<L> {
    safe_prime_with_rng(&mut OsRng, bit_length)
}

/// Checks probabilistically if the given number is prime using [`OsRng`] as the RNG.
///
/// See [`is_prime_with_rng`] for details about the performed checks.
#[cfg(feature = "default-rng")]
pub fn is_prime<const L: usize>(num: &Uint<L>) -> bool {
    is_prime_with_rng(&mut OsRng, num)
}

/// Checks probabilistically if the given number is a safe prime
/// (that is, such that `(n - 1) / 2` is also prime)
/// using [`OsRng`] as the RNG.
///
/// See [`is_prime_with_rng`] for details about the performed checks.
#[cfg(feature = "default-rng")]
pub fn is_safe_prime<const L: usize>(num: &Uint<L>) -> bool {
    is_safe_prime_with_rng(&mut OsRng, num)
}

/// Returns a random prime of size `bit_length` using the provided RNG.
///
/// Panics if `bit_length` is less than 2, or greater than the bit size of the target `Uint`.
///
/// See [`is_prime_with_rng`] for details about the performed checks.
pub fn prime_with_rng<const L: usize>(rng: &mut impl CryptoRngCore, bit_length: usize) -> Uint<L> {
    if bit_length < 2 {
        panic!("`bit_length` must be 2 or greater.");
    }
    let mut i = 0;
    loop {
        let start: Uint<L> = random_odd_uint(rng, bit_length);
        let sieve = Sieve::new(&start, bit_length, false);
        for num in sieve {
            i += 1;
            if is_prime_with_rng(rng, &num) {
                //std::println!("{i}");
                return num;
            }
        }
    }
}

pub fn unbiased_prime_with_rng<const L: usize>(rng: &mut impl CryptoRngCore, bit_length: usize) -> Uint<L> {
    if bit_length < 2 {
        panic!("`bit_length` must be 2 or greater.");
    }
    loop {
        let start: Uint<L> = random_odd_uint(rng, bit_length);
        if is_prime_with_rng(rng, &start) {
            return start;
        }
    }
}

use crypto_bigint::{NonZero, RandomMod, U1024};
use crypto_bigint::modular::runtime_mod::{DynResidue, DynResidueParams};
extern crate std;

pub fn unbiased_prime_with_rng_v2(rng: &mut impl CryptoRngCore) -> U1024 {
    // First 119 primes
    let m = U1024::from_be_hex(concat![
        "000000000000000000000000000001189c19eb3a9b2b5843bc37b89935a7674c",
        "049f9cc1a53369a9816fedfcd77c3f08e2bd5c6c2ca347f79d7084b130fe218a",
        "5db3ea4e6f59dea66d0c4f10a505f6d89012fda6074523288946aa0d99e8d3c2",
        "60e55cc290d2296309e683227097d630616907cda00e98e282ca4947b2df5fd5",
    ]);

    let lambda_m = U1024::from_be_hex(concat![
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "3296ab65ee7fc8301de8b6043a557f0e74e6896bd07763ec5f1fed0b8d17db00",
    ]);

    //let range: u128 = 0xe98c6c9a1095bdbedcabf2e91ff580;
    let range = U1024::from_be_hex(concat![
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000e98c6c9a1095bdbedcabf2e91ff580",
    ]);

    let b = U1024::random_mod(rng, &NonZero::new(m.wrapping_sub(&U1024::ONE)).unwrap()).wrapping_add(&U1024::ONE);
    let params = DynResidueParams::new(&m);
    let mut b_m = DynResidue::new(&b, params);

    let zero = DynResidue::zero(params);
    let one = DynResidue::one(params);

    let mut i = 0;
    let mut j = 0;

    loop {
        i += 1;
        //std::println!("Finding")
        let u = one - b_m.pow_bounded_exp(&lambda_m, 256);
        if u == zero {
            break;
        }

        let r = U1024::random_mod(rng, &NonZero::new(m.wrapping_sub(&U1024::ONE)).unwrap()).wrapping_add(&U1024::ONE);
        let r_m = DynResidue::new(&r, params);
        b_m = b_m + r_m * u;
    };

    let b = b_m.retrieve();
    let b_is_odd: bool = b.is_odd().into();
    let mask = U1024::MAX << 1;

    loop {
        j+=1;
        let a = U1024::random_mod(rng, &NonZero::new(range).unwrap());

        let a = if b_is_odd {
            a & mask
        }
        else {
            a | U1024::ONE
        };

        let p = a.wrapping_mul(&m).wrapping_add(&b);
        if is_prime_with_rng(rng, &p) {
            std::println!("{j}");
            return p;
        }
    }

}


/// Returns a random safe prime (that is, such that `(n - 1) / 2` is also prime)
/// of size `bit_length` using the provided RNG.
///
/// Panics if `bit_length` is less than 3, or is greater than the bit size of the target `Uint`.
///
/// See [`is_prime_with_rng`] for details about the performed checks.
pub fn safe_prime_with_rng<const L: usize>(
    rng: &mut impl CryptoRngCore,
    bit_length: usize,
) -> Uint<L> {
    if bit_length < 3 {
        panic!("`bit_length` must be 3 or greater.");
    }
    loop {
        let start: Uint<L> = random_odd_uint(rng, bit_length);
        let sieve = Sieve::new(&start, bit_length, true);
        for num in sieve {
            if is_safe_prime_with_rng(rng, &num) {
                return num;
            }
        }
    }
}

/// Checks probabilistically if the given number is prime using the provided RNG.
///
/// Performed checks:
/// - Miller-Rabin check with base 2;
/// - Strong Lucas check with Selfridge base (a.k.a. Baillie method A);
/// - Miller-Rabin check with a random base.
///
/// See [`MillerRabin`] and [`lucas_test`] for more details about the checks.
///
/// The first two checks constitute the Baillie-PSW primality test[^Baillie1980];
/// the third one is a precaution that follows the approach of GMP (as of v6.2.1).
/// At the moment of the writing there are no known composites passing
/// the Baillie-PSW test[^Baillie2021];
/// it is conjectured that they may exist, but their size is larger than the numbers
/// that are used in practice.
///
/// [^Baillie1980]: R. Baillie, S. S. Wagstaff, "Lucas pseudoprimes",
///       Math. Comp. 35 1391-1417 (1980),
///       DOI: [10.2307/2006406](https://dx.doi.org/10.2307/2006406),
///       <http://mpqs.free.fr/LucasPseudoprimes.pdf>
///
/// [^Baillie2021]: R. Baillie, A. Fiori, S. S. Wagstaff,
///       "Strengthening the Baillie-PSW primality test",
///       Math. Comp. 90 1931-1955 (2021),
///       DOI: [10.1090/mcom/3616](https://doi.org/10.1090/mcom/3616)
pub fn is_prime_with_rng<const L: usize>(rng: &mut impl CryptoRngCore, num: &Uint<L>) -> bool {
    if num == &Uint::<L>::from(2u32) {
        return true;
    }
    if num.is_even().into() {
        return false;
    }

    _is_prime_with_rng(rng, num)
}

/// Checks probabilistically if the given number is a safe prime using the provided RNG.
///
/// See [`is_prime_with_rng`] for details about the performed checks.
pub fn is_safe_prime_with_rng<const L: usize>(rng: &mut impl CryptoRngCore, num: &Uint<L>) -> bool {
    // Since, by the definition of safe prime, `(num - 1) / 2` must also be prime,
    // and therefore odd, `num` has to be equal to 3 modulo 4.
    // 5 is the only exception, so we check for it.
    if num == &Uint::<L>::from(5u32) {
        return true;
    }
    if num.as_words()[0] & 3 != 3 {
        return false;
    }

    _is_prime_with_rng(rng, num) && _is_prime_with_rng(rng, &(num >> 1))
}

/// Checks for primality assuming that `num` is odd.
fn _is_prime_with_rng<const L: usize>(rng: &mut impl CryptoRngCore, num: &Uint<L>) -> bool {
    debug_assert!(bool::from(num.is_odd()));
    let mr = MillerRabin::new(num);

    if !mr.test_base_two().is_probably_prime() {
        return false;
    }

    match lucas_test(num, SelfridgeBase, LucasCheck::Strong) {
        Primality::Composite => return false,
        Primality::Prime => return true,
        _ => {}
    }

    // The random base test only makes sense when `num > 3`.
    if num.bits() > 2 && !mr.test_random_base(rng).is_probably_prime() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crypto_bigint::{CheckedAdd, Uint, Word, U128, U64};
    use num_prime::nt_funcs::is_prime64;
    use rand_core::OsRng;

    use super::{is_prime, is_safe_prime, prime, prime_with_rng, safe_prime, safe_prime_with_rng};
    use crate::hazmat::{primes, pseudoprimes};

    fn test_large_primes<const L: usize>(nums: &[Uint<L>]) {
        for num in nums {
            assert!(is_prime(num));
        }
    }

    #[test]
    fn large_primes() {
        test_large_primes(primes::PRIMES_128);
        test_large_primes(primes::PRIMES_256);
        test_large_primes(primes::PRIMES_384);
        test_large_primes(primes::PRIMES_512);
        test_large_primes(primes::PRIMES_1024);
    }

    fn test_pseudoprimes(nums: &[u32]) {
        for num in nums {
            assert!(!is_prime(&U64::from(*num)));
        }
    }

    #[test]
    fn pseudoprimes() {
        test_pseudoprimes(pseudoprimes::EXTRA_STRONG_LUCAS);
        test_pseudoprimes(pseudoprimes::STRONG_LUCAS);
        test_pseudoprimes(pseudoprimes::ALMOST_EXTRA_STRONG_LUCAS);
        test_pseudoprimes(pseudoprimes::STRONG_BASE_2);
        test_pseudoprimes(pseudoprimes::FIBONACCI);
        test_pseudoprimes(pseudoprimes::BRUCKMAN_LUCAS);
        test_pseudoprimes(pseudoprimes::LUCAS);

        for num in pseudoprimes::STRONG_FIBONACCI {
            assert!(!is_prime(num));
        }

        assert!(!is_prime(&pseudoprimes::LARGE_CARMICHAEL_NUMBER));
    }

    fn test_cunningham_chain<const L: usize>(length: usize, num: &Uint<L>) {
        let mut next = *num;
        for i in 0..length {
            assert!(is_prime(&next));

            // The start of the chain isn't a safe prime by definition
            if i > 0 {
                assert!(is_safe_prime(&next));
            }

            next = (next << 1).checked_add(&Uint::<L>::ONE).unwrap();
        }

        // The chain ended.
        assert!(!is_prime(&next));
    }

    #[test]
    fn cunningham_chains() {
        for (length, num) in primes::CUNNINGHAM_CHAINS_128 {
            test_cunningham_chain(*length, num);
        }
        for (length, num) in primes::CUNNINGHAM_CHAINS_512 {
            test_cunningham_chain(*length, num);
        }
    }

    #[test]
    fn generate_prime() {
        for bit_length in (28..=128).step_by(10) {
            let p: U128 = prime(bit_length);
            assert!(p.bits_vartime() == bit_length);
            assert!(is_prime(&p));
        }
    }

    #[test]
    fn generate_safe_prime() {
        for bit_length in (28..=128).step_by(10) {
            let p: U128 = safe_prime(bit_length);
            assert!(p.bits_vartime() == bit_length);
            assert!(is_safe_prime(&p));
        }
    }

    #[test]
    fn corner_cases_is_prime() {
        for num in 0u64..30 {
            let is_prime_ref = is_prime64(num);
            let is_safe_prime_ref = is_prime_ref && is_prime64(num / 2);

            let num_uint = U64::from(num);
            let is_prime_test = is_prime(&num_uint);
            let is_safe_prime_test = is_safe_prime(&num_uint);

            assert_eq!(is_prime_ref, is_prime_test);
            assert_eq!(is_safe_prime_ref, is_safe_prime_test);
        }
    }

    #[test]
    fn inconclusive_sieving_result() {
        // Coverage test.
        // This number is a product of two primes larger than the maximum prime in `SMALL_PRIMES`,
        // so the initial sieving cannot tell if it is prime or not,
        // and a full primality test is run.
        assert!(!is_safe_prime(&U64::from(17881u32 * 17891u32)));
    }

    #[test]
    #[should_panic(expected = "`bit_length` must be 2 or greater")]
    fn generate_prime_too_few_bits() {
        let _p: U64 = prime_with_rng(&mut OsRng, 1);
    }

    #[test]
    #[should_panic(expected = "`bit_length` must be 3 or greater")]
    fn generate_safe_prime_too_few_bits() {
        let _p: U64 = safe_prime_with_rng(&mut OsRng, 2);
    }

    #[test]
    #[should_panic(expected = "The requested bit length (65) is larger than the chosen Uint size")]
    fn generate_prime_too_many_bits() {
        let _p: U64 = prime_with_rng(&mut OsRng, 65);
    }

    #[test]
    #[should_panic(expected = "The requested bit length (65) is larger than the chosen Uint size")]
    fn generate_safe_prime_too_many_bits() {
        let _p: U64 = safe_prime_with_rng(&mut OsRng, 65);
    }

    fn is_prime_ref(num: Word) -> bool {
        num_prime::nt_funcs::is_prime(&num, None).probably()
    }

    #[test]
    fn corner_cases_generate_prime() {
        for bits in 2usize..5 {
            for _ in 0..100 {
                let p: U64 = prime(bits);
                let p_word = p.as_words()[0];
                assert!(is_prime_ref(p_word));
            }
        }
    }

    #[test]
    fn corner_cases_generate_safe_prime() {
        for bits in 3usize..5 {
            for _ in 0..100 {
                let p: U64 = safe_prime(bits);
                let p_word = p.as_words()[0];
                assert!(is_prime_ref(p_word) && is_prime_ref(p_word / 2));
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature = "tests-openssl")]
mod tests_openssl {
    use alloc::format;

    use crypto_bigint::U128;
    use openssl::bn::{BigNum, BigNumContext};
    use rand_core::OsRng;

    use super::{is_prime, prime};
    use crate::hazmat::random_odd_uint;

    fn openssl_is_prime(num: &BigNum, ctx: &mut BigNumContext) -> bool {
        num.is_prime(64, ctx).unwrap()
    }

    fn to_openssl(num: &U128) -> BigNum {
        BigNum::from_hex_str(&format!("{num:x}")).unwrap()
    }

    fn from_openssl(num: &BigNum) -> U128 {
        U128::from_be_hex(&num.to_hex_str().unwrap())
    }

    #[test]
    fn openssl_cross_check() {
        let mut ctx = BigNumContext::new().unwrap();

        // Generate primes, let OpenSSL check them
        for _ in 0..100 {
            let p: U128 = prime(128);
            let p_bn = to_openssl(&p);
            assert!(
                openssl_is_prime(&p_bn, &mut ctx),
                "OpenSSL reports {p} as composite",
            );
        }

        // Generate primes with OpenSSL, check them
        let mut p_bn = BigNum::new().unwrap();
        for _ in 0..100 {
            p_bn.generate_prime(128, false, None, None).unwrap();
            let p = from_openssl(&p_bn);
            assert!(is_prime(&p), "we report {p} as composite");
        }

        // Generate random numbers, check if our test agrees with OpenSSL
        for _ in 0..100 {
            let p: U128 = random_odd_uint(&mut OsRng, 128);
            let actual = is_prime(&p);
            let p_bn = to_openssl(&p);
            let expected = openssl_is_prime(&p_bn, &mut ctx);
            assert_eq!(
                actual, expected,
                "difference between OpenSSL and us: OpenSSL reports {expected}, we report {actual}",
            );
        }
    }
}

#[cfg(test)]
#[cfg(feature = "tests-gmp")]
mod tests_gmp {
    use crypto_bigint::U128;
    use rand_core::OsRng;
    use rug::{
        integer::{IsPrime, Order},
        Integer,
    };

    use super::{is_prime, prime};
    use crate::hazmat::random_odd_uint;

    fn gmp_is_prime(num: &Integer) -> bool {
        matches!(num.is_probably_prime(25), IsPrime::Yes | IsPrime::Probably)
    }

    fn to_gmp(num: &U128) -> Integer {
        Integer::from_digits(num.as_words(), Order::Lsf)
    }

    fn from_gmp(num: &Integer) -> U128 {
        U128::from_words(num.to_digits(Order::Lsf).try_into().unwrap())
    }

    #[test]
    fn gmp_cross_check() {
        // Generate primes, let GMP check them
        for _ in 0..100 {
            let p: U128 = prime(128);
            let p_bn = to_gmp(&p);
            assert!(gmp_is_prime(&p_bn), "GMP reports {p} as composite");
        }

        // Generate primes with GMP, check them
        for _ in 0..100 {
            let start: U128 = random_odd_uint(&mut OsRng, 128);
            let start_bn = to_gmp(&start);
            let p_bn = start_bn.next_prime();
            let p = from_gmp(&p_bn);
            assert!(is_prime(&p), "we report {p} as composite");
        }

        // Generate random numbers, check if our test agrees with GMP
        for _ in 0..100 {
            let p: U128 = random_odd_uint(&mut OsRng, 128);
            let actual = is_prime(&p);
            let p_bn = to_gmp(&p);
            let expected = gmp_is_prime(&p_bn);
            assert_eq!(
                actual, expected,
                "difference between GMP and us: GMP reports {expected}, we report {actual}",
            );
        }
    }
}
