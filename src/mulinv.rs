extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, BigUint};
use num_traits::One;
use num_traits::Zero;

#[allow(dead_code)]
pub fn find_inverse(e: &BigUint, phi: &BigUint) -> Option<BigUint> {
    /*
    Returns the multiplicative inverse of a modulo n using extended euclidean algorithm.

    [Pseudocode] - This pseudocode may be outdated, check for changes.
    https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures

    Algorithm:
    function inverse(a, n)
    t := 0;     newt := 1
    r := n;     newr := a

    while newr ≠ 0 do
        quotient := r div newr
        (t, newt) := (newt, t − quotient × newt)
        (r, newr) := (newr, r − quotient × newr)

    if r > 1 then
        return "a is not invertible"
    if t < 0 then
        t := t + n

    return t
    */

    let mut r1 = BigInt::from(phi.clone());
    let mut r2 = BigInt::from(e.clone());
    let mut t1 = BigInt::zero();
    let mut t2 = BigInt::one();

    // Loop until r becomes zero
    while !r2.is_zero() {
        let quotient = &r1 / &r2;

        let temp_r = r2.clone();
        r2 = &r1 % &r2;
        r1 = temp_r;

        let temp_t = t2.clone();
        t2 = &t1 - (&quotient * &t2);
        t1 = temp_t;
    }

    if r1 != BigInt::one() {
        return None; // e and phi are not coprime
    }

    if t1 < BigInt::zero() {
        // return Some(BigUint::from(t1 + BigInt::from(phi.clone())));
        return Some((t1 + BigInt::from(phi.clone())).to_biguint().unwrap());
    }

    Some(t1.to_biguint().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_inverse() {
        let e = BigUint::from(65537u64);
        let phi = BigUint::from(401704021805227919u64); // Assuming n is prime for this example

        let result = find_inverse(&e, &phi).unwrap();
        println!("Multiplicative inverse: {}", result);
        // Add your own assertions based on what you expect the result to be
    }
}
