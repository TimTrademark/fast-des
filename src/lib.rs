use bitsliced_op::transpose_64x64;
use wide::u64x8;

use crate::{
    des::{compute_pc1, create_subkeys, encrypt},
    des_optimized::{compute_pc1_optimized, create_subkeys_optimized, encrypt_optimized},
};

pub mod benchmark;
mod constants;
pub mod des;
pub mod des_optimized;
pub mod sbox_optimized;
mod utils;

pub const ZERO: u64x8 = u64x8::ZERO;

pub fn des(plaintext: u64, key: u64) -> u64 {
    let (c0, d0) = compute_pc1(key);
    let subkeys = create_subkeys(c0, d0);
    let encrypted = encrypt(plaintext, subkeys);
    encrypted
}

pub fn bitsliced_des(plaintext: u64, keys: &mut [[u64; 64]; 8]) -> [[u64; 64]; 8] {
    let mut k_slice = transpose(keys);
    let (mut c0, mut d0) = compute_pc1_optimized(&mut k_slice);
    let subkeys = create_subkeys_optimized(&mut c0, &mut d0);

    encrypt_optimized(plaintext, subkeys, &mut k_slice);
    let ciphertext = transpose_back(&k_slice);
    ciphertext
}

//inline expects keys that are already transposed
pub fn bitsliced_des_inline(plaintext: u64, keys: &mut [u64x8; 64]) {
    let (mut c0, mut d0) = compute_pc1_optimized(keys);
    let subkeys = create_subkeys_optimized(&mut c0, &mut d0);

    encrypt_optimized(plaintext, subkeys, keys);
}

fn transpose(input: &[[u64; 64]; 8]) -> [u64x8; 64] {
    let mut out: [u64x8; 64] = [u64x8::ZERO; 64];
    for k in 0..8 {
        let tmp = input[k];
        let tmp = transpose_64x64(&tmp);

        for r in 0..64 {
            out[r].as_mut_array()[k] = tmp[r];
        }
    }
    out
}

fn transpose_back(input: &[u64x8; 64]) -> [[u64; 64]; 8] {
    let mut out = [[0u64; 64]; 8];

    for j in 0..64 {
        let tmp = input[j].as_array();
        out[0][j] = tmp[0];
        out[1][j] = tmp[1];
        out[2][j] = tmp[2];
        out[3][j] = tmp[3];
        out[4][j] = tmp[4];
        out[5][j] = tmp[5];
        out[6][j] = tmp[6];
        out[7][j] = tmp[7];
    }
    for k in 0..8 {
        let tmp = out[k];
        out[k] = transpose_64x64(&tmp);
    }

    out
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
        let k = 0x133457799BBCDFF1u64;
        let mut keys = [[k; 64]; 8];
        let ciphertexts = bitsliced_des(0x0123456789ABCDEF, &mut keys);
        assert_eq!(ciphertexts[0][0], 0x85E813540F0AB405);
        assert_eq!(ciphertexts[0][63], 0x85E813540F0AB405);
    }

    #[test]
    fn test_encrypt_optimized_inline_works_correctly() {
        //test data taken from https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm
        let k = 0x133457799BBCDFF1u64;
        let mut keys = [[k; 64]; 8];
        let mut transposed = transpose(&keys);
        bitsliced_des_inline(0x0123456789ABCDEF, &mut transposed);
        let ciphertexts = transpose_back(&transposed);
        assert_eq!(ciphertexts[0][0], 0x85E813540F0AB405);
        assert_eq!(ciphertexts[0][63], 0x85E813540F0AB405);
    }
}
