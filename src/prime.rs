extern crate num_bigint;
extern crate rand;

use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

#[derive(Debug)]
pub struct Prime;

impl Prime {
    #[allow(dead_code)]
    pub fn is_probably_prime(p: BigUint) -> bool {
        /*
        Rabin-Miller primality test

        [Pseudocode] - This pseudocode may be outdated, check for changes.
        https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test

        Algorithm:
        Input #1: n > 2, an odd integer to be tested for primality
        Input #2: k, the number of rounds of testing to perform
        Output: “composite” if n is found to be composite, “probably prime” otherwise

        let s > 0 and d odd > 0 such that n − 1 = 2sd  # by factoring out powers of 2 from n − 1
        repeat k times:
            a ← random(2, n − 2)  # n is always a probable prime to base 1 and n − 1
            x ← ad mod n
            repeat s times:
                y ← x2 mod n
                if y = 1 and x ≠ 1 and x ≠ n − 1 then # nontrivial square root of 1 modulo n
                    return “composite”
                x ← y
            if y ≠ 1 then
                return “composite”
        return “probably prime”
        */

        // Step 1: The number p is already provided as an argument

        let one = BigUint::one();
        let two = 2.to_biguint().unwrap();

        if p < two {
            return false;
        }

        if p == two {
            return true;
        }

        // Step 2: Find d such that 2 * d + 1 = p
        let mut d = (p.clone() - &one).clone();
        let mut r = 0;

        while &d % &two == BigUint::zero() {
            d /= &two;
            r += 1;
        }

        // Step 3: Choose a witness number y (randomly between 2 and p-2)
        // This witnesses can give us prime range to be over quadrillion numbers
        // This strong primes are better than having rounds because rounds can
        // have up to 25% chances that witness numbers generate false positives.
        let witnesses = vec![
            BigUint::from(2u32),
            BigUint::from(13u32),
            BigUint::from(23u32),
            BigUint::from(1662803u32),
        ];

        'outer: for a in witnesses {
            let mut x = a.modpow(&d, &p);

            if x == one || x == (p.clone() - &one) {
                continue;
            }

            for _ in 0..(r - 1) {
                // Step 4: Find x^2 mod p
                x = x.modpow(&two, &p);
                if x == (p.clone() - &one) {
                    continue 'outer;
                }
            }
            // definitely composite
            return false;
        }
        // probably prime
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Prime;
    use num_bigint::BigUint;

    #[test]
    fn test_is_probably_prime() {
        let p = BigUint::from(747u32);
        assert_eq!(Prime::is_probably_prime(p), false);

        let p = BigUint::from(743u32);
        assert_eq!(Prime::is_probably_prime(p), true);
    }
}
