use crate::{
    des::{compute_pc1, create_subkeys, encrypt},
    des_optimized::{
        compute_pc1_optimized, create_subkeys_optimized, encrypt_optimized,
        transpose_bitsliced_to_u64, transpose_u64_to_bitsliced,
    },
};

pub mod benchmark;
mod constants;
pub mod des;
pub mod des_optimized;
pub mod sbox_optimized;
mod utils;

pub fn des(plaintext: u64, key: u64) -> u64 {
    let (c0, d0) = compute_pc1(key);
    let subkeys = create_subkeys(c0, d0);
    let encrypted = encrypt(plaintext, subkeys);
    encrypted
}

pub fn des_optimized(plaintext: u64, keys: &[u64; 64]) -> [u64; 64] {
    let k_slice = transpose_u64_to_bitsliced(keys);
    let (mut c0, mut d0) = compute_pc1_optimized(k_slice);
    let subkeys = create_subkeys_optimized(&mut c0, &mut d0);

    let encrypted = encrypt_optimized(plaintext, subkeys);
    let ciphertext = transpose_bitsliced_to_u64(&encrypted);
    ciphertext
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_encrypt_works_correctly() {
        //test data taken from https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm
        let ciphertext = des(0x0123456789ABCDEF, 0x133457799BBCDFF1);
        assert_eq!(ciphertext, 0x85E813540F0AB405);
    }

    #[test]
    fn test_encrypt_optimized_works_correctly() {
        //test data taken from https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm
        let keys = [0x133457799BBCDFF1u64; 64];
        let ciphertexts = des_optimized(0x0123456789ABCDEF, &keys);
        assert_eq!(ciphertexts[0], 0x85E813540F0AB405);
        assert_eq!(ciphertexts[63], 0x85E813540F0AB405);
    }
}
