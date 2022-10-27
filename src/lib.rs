pub mod web_rand;

use std::str::FromStr;

use js_sys::BigInt;
use num_bigint_dig::{prime::probably_prime, BigUint, RandPrime};
use rand::RngCore;
use rand_chacha::{rand_core::SeedableRng, ChaCha12Rng};
use wasm_bindgen::prelude::wasm_bindgen;
use web_rand::WebRand;

fn gen_safe_prime_rust<R: RngCore>(rng: &mut R, bits: usize) -> BigUint {
    if bits < 3 {
        panic!("Cannot generate safe prime number under 3 bits");
    }
    let one = BigUint::from(1u8);
    loop {
        // generate a prime number p
        let mut prime = rng.gen_prime(bits - 1);
        // then check if 2p + 1 is prime
        prime <<= 1;
        prime |= &one;
        if probably_prime(&prime, 20) {
            return prime;
        }
    }
}

#[wasm_bindgen(js_name = "genSafePrime")]
pub fn gen_safe_prime(bits: usize) -> BigInt {
    let mut rng = ChaCha12Rng::from_entropy();
    let prime = gen_safe_prime_rust(&mut rng, bits);
    convert_to_bigint(prime)
}

fn convert_to_bigint(biguint: BigUint) -> BigInt {
    BigInt::from_str(&biguint.to_string()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bits = 1024;
        let prime = gen_safe_prime_rust(&mut ChaCha12Rng::from_entropy(), bits);
        assert_eq!(bits, prime.bits());
    }
}
