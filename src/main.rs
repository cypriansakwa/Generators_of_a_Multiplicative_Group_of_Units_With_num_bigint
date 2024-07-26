use num_bigint::{BigUint};
use num_integer::Integer;
use num_traits::{One};
use std::collections::HashSet;

// Compute Euler's Totient Function φ(n)
fn euler_totient(n: &BigUint) -> BigUint {
    let mut result = n.clone();
    let mut p = BigUint::from(2u32);
    let mut temp_n = n.clone(); // Create a mutable copy of n for modification
    while &p * &p <= temp_n {
        if temp_n.is_multiple_of(&p) {
            while temp_n.is_multiple_of(&p) {
                temp_n /= &p;
            }
            result -= &result / &p;
        }
        p += 1u32;
    }
    if temp_n > BigUint::from(1u32) {
        result -= &result / &temp_n;
    }
    result
}

// Check if a number is a generator of Z_n^*
fn is_generator(g: &BigUint, n: &BigUint, phi_n: &BigUint, factors: &HashSet<BigUint>) -> bool {
    for factor in factors {
        let exponent = phi_n / factor;
        if g.modpow(&exponent, n) == BigUint::one() {
            return false;
        }
    }
    true
}

// Get the prime factors of a number
fn prime_factors(n: &BigUint) -> HashSet<BigUint> {
    let mut factors = HashSet::new();
    let mut num = n.clone();
    let mut p = BigUint::from(2u32);
    while &p * &p <= num {
        if num.is_multiple_of(&p) {
            factors.insert(p.clone());
            while num.is_multiple_of(&p) {
                num /= &p;
            }
        }
        p += 1u32;
    }
    if num > BigUint::from(1u32) {
        factors.insert(num);
    }
    factors
}

fn main() {
    let n = BigUint::parse_bytes(b"22", 10).unwrap();  // Change this value to test with different n
    let phi_n = euler_totient(&n);
    let factors = prime_factors(&phi_n);

    let mut generators = Vec::new();
    let mut candidate = BigUint::from(2u32);
    while &candidate < &n {
        if candidate.gcd(&n) == BigUint::one() && is_generator(&candidate, &n, &phi_n, &factors) {
            generators.push(candidate.clone());
        }
        candidate += BigUint::one();
    }

    println!("Generators of the multiplicative group Z_{}^*: {:?}", n, generators);
}



