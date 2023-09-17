extern crate num_bigint;
use num_bigint::RandBigInt;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, ToPrimitive, Zero};

use crate::mulinv::find_inverse;
use crate::prime::Prime;

#[derive(Debug)]
pub struct RSA {
    // Product of two random prime numbers `p` and `q`
    n: BigUint,

    // Private key
    d: BigUint,

    // Public key
    e: BigUint,
}

// Function to find the next prime greater than n
fn find_next_prime(n: BigUint) -> BigUint {
    let mut candidate = n;
    loop {
        if Prime::is_probably_prime(candidate.clone()) {
            return candidate;
        }
        candidate += BigUint::one();
    }
}

impl RSA {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        println!("Finding Big Prime numbers");

        // Generate large random numbers for p and q
        let p: BigUint = rng.gen_biguint(512);
        let q: BigUint = rng.gen_biguint(512);

        // Find the next primes greater than p and q
        let p_prime = find_next_prime(p);
        let q_prime = find_next_prime(q);

        let n = &p_prime * &q_prime;
        println!("Calculating N = P * Q = {}", n);
        println!("Generating key pair...");
        let phi = (&p_prime - BigUint::one()) * (&q_prime - BigUint::one());
        let e = 0x10001.to_biguint().unwrap();
        println!("* Public key is:  {}", e);
        let d = find_inverse(&e, &phi).expect("could not calculate private key");
        println!("* Private key is: {}", d);
        Self { n, d, e }
    }

    // Function to calculate a^b mod m using the Square and Multiply algorithm
    pub fn square_and_multiply(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
        /*
        Square and multiply algorithm
        This is an efficient way to calculate without multiplying powers of numbers
        Rough idea:
        1. 3^45 mod 7, where 45 can be converted to digital form as 101101
        2. given properties x^1 * x^1 = x^10, x^10*x^10 = x^100, x^100*x = x^101
        3. 3^1*3*1 = 3^10 which results as 3^2, hence 2
        4. since we have 3^10, so now 3^10*3^10 = 3^ 100 which results as 3^4, hence 4
        5. similarly calculating all powers for value 10110 using properties in #2, final value will be 3^101101 which will result in 3^45, hence 6
        6. Final result hence for 3^45 mod 7 = 6
        */
        // Initialize result to 1
        let mut result = BigUint::one();
        // Calculate base = a mod m
        let mut base = a.clone() % m;
        // Clone the exponent for manipulation
        let mut exponent = b.clone();

        // Loop until the exponent becomes zero
        while exponent > BigUint::zero() {
            // If the exponent is odd, multiply the result by base mod m
            if &exponent % BigUint::from(2u32) == BigUint::one() {
                result = (result * &base) % m;
            }
            // Right shift the exponent by 1 (divide by 2)
            exponent >>= 1;
            // Square the base mod m
            base = (&base * &base) % m;
        }

        // Return the final result
        result
    }

    // Encryption function
    pub fn encrypt(&self, msg: String) -> Vec<BigUint> {
        // Map each character in the message to its ASCII value
        // Then encrypt it using square_and_multiply
        msg.chars()
            .map(|c| {
                let ascii_val = BigUint::from(c as u32);
                Self::square_and_multiply(&ascii_val, &self.e, &self.n)
            })
            .collect()
    }
    // Decryption function
    pub fn decrypt(&self, ciphertext: Vec<BigUint>) -> String {
        ciphertext
            .iter()
            .map(|c| {
                let decrypted_val = Self::square_and_multiply(c, &self.d, &self.n);
                (decrypted_val.to_u32().unwrap() as u8) as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let rsa = RSA::new();
        let msg = "Hello world!".to_string();

        let decrypt = rsa.decrypt(rsa.encrypt(msg.clone()));
        assert_eq!(decrypt, msg);
    }
}
