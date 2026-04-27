use bitsliced_op::{ALL_ONES, transpose_64x64};
use core::arch::x86_64::__m512i;
use std::{
    arch::x86_64::{
        __m256i, _mm256_set_epi64x, _mm256_set1_epi64x, _mm256_storeu_si256, _mm512_set_epi64,
        _mm512_set1_epi64, _mm512_setzero_si512, _mm512_storeu_si512,
    },
    mem::transmute,
};
use wide::u64x8;

use crate::{
    constants::IP_INVO,
    des::{compute_pc1, create_subkeys, encrypt},
    des_optimized::{
        compute_pc1_optimized, create_subkeys_optimized, encrypt_avx_2, encrypt_avx_512,
        encrypt_optimized, encrypt_simd, feistel_avx_512,
    },
};

pub mod benchmark;
mod constants;
pub mod des;
pub mod des_optimized;
pub mod sboxes;
mod utils;

pub const ZERO: u64x8 = u64x8::ZERO;

pub fn des(plaintext: u64, key: u64) -> u64 {
    let (c0, d0) = compute_pc1(key);
    let subkeys = create_subkeys(c0, d0);
    let encrypted = encrypt(plaintext, subkeys);
    encrypted
}

//SIMD
pub fn bitsliced_des_simd(plaintext: u64, keys: &[[u64; 64]; 8]) -> [[u64; 64]; 8] {
    let mut k_slice = transpose(keys);

    encrypt_simd(plaintext, &mut k_slice);
    let ciphertext = transpose_back(&k_slice);
    ciphertext
}

//inline expects keys that are already transposed
pub fn bitsliced_des_inline_simd(plaintext: u64, keys: &mut [u64x8; 64]) {
    encrypt_simd(plaintext, keys);
}

//expects plaintexts of 56bits, 8 MSB of all u64's are ignored
pub fn bitsliced_netntlmv1_simd(plaintext: u64, keys: &[[u64; 64]; 8]) -> [[u64; 64]; 8] {
    let transposed = transpose(keys);
    let mut keys = convert_to_key::<u64x8>(&transposed, ALL_ONES);
    bitsliced_des_inline_simd(plaintext, &mut keys);
    let ciphertext = transpose_back(&keys);
    ciphertext
}

//expects plaintexts of 56bits, 8 MSB of all u64's are ignored
pub fn bitsliced_netntlmv1_inline_simd(plaintext: u64, keys: &mut [u64x8; 64]) {
    *keys = convert_to_key::<u64x8>(&keys, ALL_ONES);
    bitsliced_des_inline_simd(plaintext, keys);
}

//AVX512
#[target_feature(enable = "avx512f,avx512vl,avx512bw")]
pub unsafe fn bitsliced_des_simd_avx_512(keys: &[[u64; 64]; 8]) -> [[u64; 64]; 8] {
    unsafe {
        let mut k_slice = transpose_avx_512(keys);
        encrypt_avx_512(&mut k_slice);
        transpose_back_avx_512(&k_slice)
    }
}

#[target_feature(enable = "avx512f,avx512vl,avx512bw")]
pub unsafe fn bitsliced_des_inline_simd_avx_512(keys: &mut [__m512i; 64]) {
    encrypt_avx_512(keys);
}

#[target_feature(enable = "avx512f,avx512vl,avx512bw")]
pub unsafe fn bitsliced_netntlmv1_inline_simd_avx_512(keys: &mut [__m512i; 64]) {
    *keys = convert_to_key::<__m512i>(&keys, _mm512_set1_epi64(-1i64));
    bitsliced_des_inline_simd_avx_512(keys);
}

#[target_feature(enable = "avx512f,avx512vl,avx512bw")]
pub unsafe fn bitsliced_netntlmv1_simd_avx_512(keys: &[[u64; 64]; 8]) -> [[u64; 64]; 8] {
    unsafe {
        let k_slice = transpose_avx_512(keys);
        let mut converted = convert_to_key::<__m512i>(&k_slice, _mm512_set1_epi64(-1i64));
        encrypt_avx_512(&mut converted);
        transpose_back_avx_512(&converted)
    }
}

//AVX2
#[target_feature(enable = "avx2")]
pub unsafe fn bitsliced_des_simd_avx_2(keys: &[[u64; 64]; 4]) -> [[u64; 64]; 4] {
    unsafe {
        let mut k_slice = transpose_avx_2(keys);
        encrypt_avx_2(&mut k_slice);
        transpose_back_avx_2(&k_slice)
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn bitsliced_des_inline_simd_avx_2(keys: &mut [__m256i; 64]) {
    encrypt_avx_2(keys);
}

#[target_feature(enable = "avx2")]
pub unsafe fn bitsliced_netntlmv1_simd_avx_2(keys: &[[u64; 64]; 4]) -> [[u64; 64]; 4] {
    unsafe {
        let k_slice = transpose_avx_2(keys);
        let mut converted = convert_to_key::<__m256i>(&k_slice, _mm256_set1_epi64x(-1i64));
        encrypt_avx_2(&mut converted);
        transpose_back_avx_2(&converted)
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn bitsliced_netntlmv1_inline_simd_avx_2(keys: &mut [__m256i; 64]) {
    *keys = convert_to_key::<__m256i>(&keys, _mm256_set1_epi64x(-1i64));
    bitsliced_des_inline_simd_avx_2(keys);
}

//verbose for better performance
fn convert_to_key<T: Copy>(plaintexts: &[T; 64], all_ones: T) -> [T; 64] {
    [
        plaintexts[8],
        plaintexts[9],
        plaintexts[10],
        plaintexts[11],
        plaintexts[12],
        plaintexts[13],
        plaintexts[14],
        all_ones,
        plaintexts[15],
        plaintexts[16],
        plaintexts[17],
        plaintexts[18],
        plaintexts[19],
        plaintexts[20],
        plaintexts[21],
        all_ones,
        plaintexts[22],
        plaintexts[23],
        plaintexts[24],
        plaintexts[25],
        plaintexts[26],
        plaintexts[27],
        plaintexts[28],
        all_ones,
        plaintexts[29],
        plaintexts[30],
        plaintexts[31],
        plaintexts[32],
        plaintexts[33],
        plaintexts[34],
        plaintexts[35],
        all_ones,
        plaintexts[36],
        plaintexts[37],
        plaintexts[38],
        plaintexts[39],
        plaintexts[40],
        plaintexts[41],
        plaintexts[42],
        all_ones,
        plaintexts[43],
        plaintexts[44],
        plaintexts[45],
        plaintexts[46],
        plaintexts[47],
        plaintexts[48],
        plaintexts[49],
        all_ones,
        plaintexts[50],
        plaintexts[51],
        plaintexts[52],
        plaintexts[53],
        plaintexts[54],
        plaintexts[55],
        plaintexts[56],
        all_ones,
        plaintexts[57],
        plaintexts[58],
        plaintexts[59],
        plaintexts[60],
        plaintexts[61],
        plaintexts[62],
        plaintexts[63],
        all_ones,
    ]
}

//TODO: optimize AVX transpose
unsafe fn transpose_avx_512(blocks: &[[u64; 64]; 8]) -> [__m512i; 64] {
    let mut outputs = [transmute([0u64; 8]); 64];
    let mut blocks_transposed = [[0u64; 64]; 8];

    // 1. Transpose each block individually using your existing function
    for b in 0..8 {
        blocks_transposed[b] = bitsliced_op::transpose_64x64(&blocks[b]);
    }
    for bit_idx in 0..64 {
        outputs[bit_idx] = _mm512_set_epi64(
            blocks_transposed[7][bit_idx] as i64,
            blocks_transposed[6][bit_idx] as i64,
            blocks_transposed[5][bit_idx] as i64,
            blocks_transposed[4][bit_idx] as i64,
            blocks_transposed[3][bit_idx] as i64,
            blocks_transposed[2][bit_idx] as i64,
            blocks_transposed[1][bit_idx] as i64,
            blocks_transposed[0][bit_idx] as i64,
        );
    }

    outputs
}

unsafe fn transpose_back_avx_512(outputs_simd: &[__m512i; 64]) -> [[u64; 64]; 8] {
    let mut blocks_transposed = [[0u64; 64]; 8];
    let mut final_blocks = [[0u64; 64]; 8];

    // 1. Extract the lanes back into 8 intermediate blocks
    // Each register contains Bit i for all 8 blocks.
    for bit_idx in 0..64 {
        let reg = outputs_simd[bit_idx];

        // Use store to get the data out of the SIMD register into a temporary buffer
        let mut lanes = [0u64; 8];
        _mm512_storeu_si512(lanes.as_mut_ptr() as *mut _, reg);

        // Map lanes back to their respective blocks
        // Note: set_epi64(7, 6, 5, 4, 3, 2, 1, 0) stores as [0, 1, 2, 3, 4, 5, 6, 7] in memory
        for b in 0..8 {
            blocks_transposed[b][bit_idx] = lanes[b];
        }
    }

    // 2. Transpose each block one last time to return to original row-major format
    for b in 0..8 {
        final_blocks[b] = bitsliced_op::transpose_64x64(&blocks_transposed[b]);
    }

    final_blocks
}

//TODO: optimize AVX transpose
unsafe fn transpose_avx_2(blocks: &[[u64; 64]; 4]) -> [__m256i; 64] {
    let mut outputs = [transmute([0u64; 4]); 64];
    let mut blocks_transposed = [[0u64; 64]; 4];

    // 1. Transpose each block individually using your existing function
    for b in 0..4 {
        blocks_transposed[b] = bitsliced_op::transpose_64x64(&blocks[b]);
    }
    for bit_idx in 0..64 {
        outputs[bit_idx] = _mm256_set_epi64x(
            blocks_transposed[3][bit_idx] as i64,
            blocks_transposed[2][bit_idx] as i64,
            blocks_transposed[1][bit_idx] as i64,
            blocks_transposed[0][bit_idx] as i64,
        );
    }

    outputs
}

unsafe fn transpose_back_avx_2(outputs_simd: &[__m256i; 64]) -> [[u64; 64]; 4] {
    let mut blocks_transposed = [[0u64; 64]; 4];
    let mut final_blocks = [[0u64; 64]; 4];

    // 1. Extract the lanes back into 8 intermediate blocks
    // Each register contains Bit i for all 8 blocks.
    for bit_idx in 0..64 {
        let reg = outputs_simd[bit_idx];

        // Use store to get the data out of the SIMD register into a temporary buffer
        let mut lanes = [0u64; 4];
        _mm256_storeu_si256(lanes.as_mut_ptr() as *mut _, reg);

        // Map lanes back to their respective blocks
        // Note: set_epi64(7, 6, 5, 4, 3, 2, 1, 0) stores as [0, 1, 2, 3, 4, 5, 6, 7] in memory
        for b in 0..4 {
            blocks_transposed[b][bit_idx] = lanes[b];
        }
    }

    // 2. Transpose each block one last time to return to original row-major format
    for b in 0..4 {
        final_blocks[b] = bitsliced_op::transpose_64x64(&blocks_transposed[b]);
    }

    final_blocks
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

    use std::arch::x86_64::{_mm512_set1_epi64, _mm512_setzero_si512};

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
    fn test_encrypt_optimized_inline_avx_512_works_correctly() {
        unsafe {
            let k = 0x8923BDFDAF753F63u64;
            let keys = [k; 64];
            let transposed = transpose_64x64(&keys);
            let mut keys = [_mm512_setzero_si512(); 64];
            for (i, t) in transposed.iter().enumerate() {
                if *t != 0 {
                    keys[i] = _mm512_set1_epi64(-1);
                }
            }
            bitsliced_des_inline_simd_avx_512(&mut keys);
            let mut output = Box::new([0u64; 64]);
            for (i, k) in keys.iter().enumerate() {
                let lanes: [u64; 8] = transmute(*k);
                let first = lanes[0];
                output[i] = first;
            }
            let ciphertexts = transpose_64x64(&output);
            assert_eq!(ciphertexts[0], 0x727B4E35F947129E);
            assert_eq!(ciphertexts[0], 0x727B4E35F947129E);
        }
    }

    #[test]
    fn test_encrypt_optimized_avx_512_works_correctly() {
        unsafe {
            let k = 0x8923BDFDAF753F63u64;
            let keys = [[k; 64]; 8];

            let output = bitsliced_des_simd_avx_512(&keys);
            assert_eq!(output[0][0], 0x727B4E35F947129E);
            assert_eq!(output[7][63], 0x727B4E35F947129E);
        }
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

    #[test]
    fn test_netntlmv1_encrypt_avx_512_works_correctly() {
        unsafe {
            let k = 0x8846F7EAEE8FB1u64;
            let keys = [[k; 64]; 8];
            let ciphertexts = bitsliced_netntlmv1_simd_avx_512(&keys);
            assert_eq!(ciphertexts[0][0], 0x727B4E35F947129E);
            assert_eq!(ciphertexts[0][63], 0x727B4E35F947129E);
        }
    }

    #[test]
    fn test_netntlmv1_inline_encrypt_avx_512_works_correctly() {
        unsafe {
            let k = 0x8846F7EAEE8FB1u64;
            let keys = [[k; 64]; 8];
            let mut transposed = transpose_avx_512(&keys);
            bitsliced_netntlmv1_inline_simd_avx_512(&mut transposed);
            let ciphertexts = transpose_back_avx_512(&transposed);
            assert_eq!(ciphertexts[0][0], 0x727B4E35F947129E);
            assert_eq!(ciphertexts[7][63], 0x727B4E35F947129E);
        }
    }

    #[test]
    fn test_netntlmv1_inline_encrypt_avx_2_works_correctly() {
        unsafe {
            let k = 0x8846F7EAEE8FB1u64;
            let keys = [[k; 64]; 4];
            let mut transposed = transpose_avx_2(&keys);
            bitsliced_netntlmv1_inline_simd_avx_2(&mut transposed);
            let ciphertexts = transpose_back_avx_2(&transposed);
            assert_eq!(ciphertexts[0][0], 0x727B4E35F947129E);
            assert_eq!(ciphertexts[3][63], 0x727B4E35F947129E);
        }
    }
}
