use bitsliced_op::{ALL_ONES, transpose_64x64};
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

pub fn bitsliced_des_simd(plaintext: u64, keys: &[[u64; 64]; 8]) -> [[u64; 64]; 8] {
    let mut k_slice = transpose(keys);
    let (mut c0, mut d0) = compute_pc1_optimized(&mut k_slice);
    let subkeys = create_subkeys_optimized(&mut c0, &mut d0);

    encrypt_optimized(plaintext, subkeys, &mut k_slice);
    let ciphertext = transpose_back(&k_slice);
    ciphertext
}

//inline expects keys that are already transposed
pub fn bitsliced_des_inline_simd(plaintext: u64, keys: &mut [u64x8; 64]) {
    let (mut c0, mut d0) = compute_pc1_optimized(keys);
    let subkeys = create_subkeys_optimized(&mut c0, &mut d0);

    encrypt_optimized(plaintext, subkeys, keys);
}

//expects plaintexts of 56bits, 8 MSB of all u64's are ignored
pub fn bitsliced_netntlmv1_simd(plaintext: u64, keys: &[[u64; 64]; 8]) -> [[u64; 64]; 8] {
    let transposed = transpose(keys);
    let mut keys = convert_to_key(&transposed);
    bitsliced_des_inline_simd(plaintext, &mut keys);
    let ciphertext = transpose_back(&keys);
    ciphertext
}

//expects plaintexts of 56bits, 8 MSB of all u64's are ignored
pub fn bitsliced_netntlmv1_inline_simd(plaintext: u64, keys: &mut [u64x8; 64]) {
    convert_to_key_inline(keys);
    bitsliced_des_inline_simd(plaintext, keys);
}

const DES_KEY_PERM: [i8; 64] = [
    0, 1, 2, 3, 4, 5, 6, -1, 7, 8, 9, 10, 11, 12, 13, -1, 14, 15, 16, 17, 18, 19, 20, -1, 21, 22,
    23, 24, 25, 26, 27, -1, 28, 29, 30, 31, 32, 33, 34, -1, 35, 36, 37, 38, 39, 40, 41, -1, 42, 43,
    44, 45, 46, 47, 48, -1, 49, 50, 51, 52, 53, 54, 55, -1,
];

fn convert_to_key(plaintexts: &[u64x8; 64]) -> [u64x8; 64] {
    let mut out = [ZERO; 64];
    for i in 0..64 {
        let src = DES_KEY_PERM[i];
        if src >= 0 {
            out[i] = plaintexts[(src as usize) + 8];
        } else {
            out[i] = ALL_ONES;
        }
    }
    out
}

fn convert_to_key_inline(plaintexts: &mut [u64x8; 64]) {
    let mut tmp = [ZERO; 56];
    for i in 8..64 {
        tmp[i - 8] = plaintexts[i];
    }
    for i in 0..64 {
        let src = DES_KEY_PERM[i];
        if src >= 0 {
            plaintexts[i] = tmp[src as usize];
        } else {
            plaintexts[i] = ALL_ONES;
        }
    }
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
        let ciphertexts = bitsliced_des_simd(0x0123456789ABCDEF, &mut keys);
        assert_eq!(ciphertexts[0][0], 0x85E813540F0AB405);
        assert_eq!(ciphertexts[0][63], 0x85E813540F0AB405);
    }

    #[test]
    fn test_encrypt_optimized_inline_works_correctly() {
        //test data taken from https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm
        let k = 0x133457799BBCDFF1u64;
        let mut keys = [[k; 64]; 8];
        let mut transposed = transpose(&keys);
        bitsliced_des_inline_simd(0x0123456789ABCDEF, &mut transposed);
        let ciphertexts = transpose_back(&transposed);
        assert_eq!(ciphertexts[0][0], 0x85E813540F0AB405);
        assert_eq!(ciphertexts[0][63], 0x85E813540F0AB405);
    }

    #[test]
    fn test_netntlmv1_encrypt_works_correctly() {
        let k = 0x8846F7EAEE8FB1u64;
        let keys = [[k; 64]; 8];
        let ciphertexts = bitsliced_netntlmv1_simd(0x1122334455667788, &keys);
        assert_eq!(ciphertexts[0][0], 0x727B4E35F947129E);
        assert_eq!(ciphertexts[0][63], 0x727B4E35F947129E);
    }

    #[test]
    fn test_netntlmv1_inline_encrypt_works_correctly() {
        let k = 0x8846F7EAEE8FB1u64;
        let keys = [[k; 64]; 8];
        let mut transposed = transpose(&keys);
        bitsliced_netntlmv1_inline_simd(0x1122334455667788, &mut transposed);
        let ciphertexts = transpose_back(&transposed);
        assert_eq!(ciphertexts[0][0], 0x727B4E35F947129E);
        assert_eq!(ciphertexts[0][63], 0x727B4E35F947129E);
    }
}
