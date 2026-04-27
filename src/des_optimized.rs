use std::{
    arch::x86_64::{__m256i, __m512i, _mm256_xor_si256, _mm512_xor_si512},
    mem::transmute,
    ops::{BitAnd, BitOr, Shl, Shr},
};

use crate::{
    ZERO,
    constants::{EO, IP, IP_INVO, PC_1O, PC_2O, PO},
    sboxes::{
        sbox_avx2::{
            s1_avx_2, s2_avx_2, s3_avx_2, s4_avx_2, s5_avx_2, s6_avx_2, s7_avx_2, s8_avx_2,
        },
        sbox_avx512::{
            s1_avx_512, s2_avx_512, s3_avx_512, s4_avx_512, s5_avx_512, s6_avx_512, s7_avx_512,
            s8_avx_512,
        },
        sbox_optimized::{s1, s2, s3, s4, s5, s6, s7, s8},
    },
};
use bitsliced_op::transpose_64x64;
use wide::u64x8;

//slice[i] contains bit with index i of all 64 slices
// a column is equal to one item/number
pub fn compute_pc1_optimized(k_slices: &[u64x8; 64]) -> ([u64x8; 28], [u64x8; 28]) {
    let mut c0 = [ZERO; 28];
    let mut d0 = [ZERO; 28];

    for (i, &src) in PC_1O.iter().enumerate() {
        if i < 28 {
            c0[i] = k_slices[src as usize];
        } else {
            d0[i - 28] = k_slices[src as usize];
        }
    }

    (c0, d0)
}

const SHIFTS: [u32; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

pub fn create_subkeys_optimized(c0: &mut [u64x8; 28], d0: &mut [u64x8; 28]) -> [[u64x8; 48]; 16] {
    let mut subkeys = [[ZERO; 48]; 16];

    for (index, s) in SHIFTS.iter().enumerate() {
        c0.rotate_left(*s as usize);
        d0.rotate_left(*s as usize);
        for (i, &src) in PC_2O.iter().enumerate() {
            if src < 28 {
                subkeys[index][i] = c0[src as usize]
            } else {
                subkeys[index][i] = d0[(src - 28) as usize];
            }
        }
    }

    subkeys
}

//TODO: create a version where IP is precomputed
pub fn encrypt_optimized(plaintext: u64, subkeys: [[u64x8; 48]; 16], output: &mut [u64x8; 64]) {
    let ip = permute_bits_pc(&IP, plaintext, 64);
    //repeat and transpose IP
    let ip_bitsliced = transpose_64x64(&[ip; 64]);
    let mut l: [u64x8; 32] = std::array::from_fn(|i| u64x8::splat(ip_bitsliced[i]));
    let mut r: [u64x8; 32] = std::array::from_fn(|i| u64x8::splat(ip_bitsliced[i + 32]));

    for i in 0..16 {
        let (new_l, new_r) = feistel_function_optimized(l, r, subkeys[i]);
        l = new_l;
        r = new_r;
    }
    let mut r_l = [ZERO; 64];
    r_l[..32].copy_from_slice(&r);
    r_l[32..].copy_from_slice(&l);
    for (i, &src) in IP_INVO.iter().enumerate() {
        output[i] = r_l[src as usize];
    }
}

#[inline(always)]
fn feistel_function_optimized(
    l: [u64x8; 32],
    r: [u64x8; 32],
    subkey: [u64x8; 48],
) -> ([u64x8; 32], [u64x8; 32]) {
    let new_l = r;
    let mut e = [ZERO; 48];
    for (i, &src) in EO.iter().enumerate() {
        e[i] = r[src as usize];
    }
    for i in 0..48 {
        e[i] ^= subkey[i];
    }
    let mut output: [u64x8; 32] = [ZERO; 32];

    let s1_output = s1(e[0], e[1], e[2], e[3], e[4], e[5]);
    let s2_output = s2(e[6], e[7], e[8], e[9], e[10], e[11]);
    let s3_output = s3(e[12], e[13], e[14], e[15], e[16], e[17]);
    let s4_output = s4(e[18], e[19], e[20], e[21], e[22], e[23]);
    let s5_output = s5(e[24], e[25], e[26], e[27], e[28], e[29]);
    let s6_output = s6(e[30], e[31], e[32], e[33], e[34], e[35]);
    let s7_output = s7(e[36], e[37], e[38], e[39], e[40], e[41]);
    let s8_output = s8(e[42], e[43], e[44], e[45], e[46], e[47]);
    // s1
    output[0] = s1_output.0;
    output[1] = s1_output.1;
    output[2] = s1_output.2;
    output[3] = s1_output.3;

    // s2
    output[4] = s2_output.0;
    output[5] = s2_output.1;
    output[6] = s2_output.2;
    output[7] = s2_output.3;

    // s3
    output[8] = s3_output.0;
    output[9] = s3_output.1;
    output[10] = s3_output.2;
    output[11] = s3_output.3;

    // s4
    output[12] = s4_output.0;
    output[13] = s4_output.1;
    output[14] = s4_output.2;
    output[15] = s4_output.3;

    // s5
    output[16] = s5_output.0;
    output[17] = s5_output.1;
    output[18] = s5_output.2;
    output[19] = s5_output.3;

    // s6
    output[20] = s6_output.0;
    output[21] = s6_output.1;
    output[22] = s6_output.2;
    output[23] = s6_output.3;

    // s7
    output[24] = s7_output.0;
    output[25] = s7_output.1;
    output[26] = s7_output.2;
    output[27] = s7_output.3;

    // s8
    output[28] = s8_output.0;
    output[29] = s8_output.1;
    output[30] = s8_output.2;
    output[31] = s8_output.3;
    let mut new_r = [ZERO; 32];
    for (i, &src) in PO.iter().enumerate() {
        new_r[i] = output[src as usize];
    }
    //inline array modification to avoid extra allocation
    for i in 0..32 {
        new_r[i] = l[i] ^ new_r[i];
    }
    (new_l, new_r)
}

const SUBKEY_SCHEDULE: [[usize; 48]; 16] = [
    //0:
    [
        9, 50, 33, 59, 48, 16, 32, 56, 1, 8, 18, 41, 2, 34, 25, 24, 43, 57, 58, 0, 35, 26, 17, 40,
        21, 27, 38, 53, 36, 3, 46, 29, 4, 52, 22, 28, 60, 20, 37, 62, 14, 19, 44, 13, 12, 61, 54,
        30,
    ],
    //1:
    [
        1, 42, 25, 51, 40, 8, 24, 48, 58, 0, 10, 33, 59, 26, 17, 16, 35, 49, 50, 57, 56, 18, 9, 32,
        13, 19, 30, 45, 28, 62, 38, 21, 27, 44, 14, 20, 52, 12, 29, 54, 6, 11, 36, 5, 4, 53, 46,
        22,
    ],
    //2:
    [
        50, 26, 9, 35, 24, 57, 8, 32, 42, 49, 59, 17, 43, 10, 1, 0, 48, 33, 34, 41, 40, 2, 58, 16,
        60, 3, 14, 29, 12, 46, 22, 5, 11, 28, 61, 4, 36, 27, 13, 38, 53, 62, 20, 52, 19, 37, 30, 6,
    ],
    //3:
    [
        34, 10, 58, 48, 8, 41, 57, 16, 26, 33, 43, 1, 56, 59, 50, 49, 32, 17, 18, 25, 24, 51, 42,
        0, 44, 54, 61, 13, 27, 30, 6, 52, 62, 12, 45, 19, 20, 11, 60, 22, 37, 46, 4, 36, 3, 21, 14,
        53,
    ],
    //4:
    [
        18, 59, 42, 32, 57, 25, 41, 0, 10, 17, 56, 50, 40, 43, 34, 33, 16, 1, 2, 9, 8, 35, 26, 49,
        28, 38, 45, 60, 11, 14, 53, 36, 46, 27, 29, 3, 4, 62, 44, 6, 21, 30, 19, 20, 54, 5, 61, 37,
    ],
    //5:
    [
        2, 43, 26, 16, 41, 9, 25, 49, 59, 1, 40, 34, 24, 56, 18, 17, 0, 50, 51, 58, 57, 48, 10, 33,
        12, 22, 29, 44, 62, 61, 37, 20, 30, 11, 13, 54, 19, 46, 28, 53, 5, 14, 3, 4, 38, 52, 45,
        21,
    ],
    //6:
    [
        51, 56, 10, 0, 25, 58, 9, 33, 43, 50, 24, 18, 8, 40, 2, 1, 49, 34, 35, 42, 41, 32, 59, 17,
        27, 6, 13, 28, 46, 45, 21, 4, 14, 62, 60, 38, 3, 30, 12, 37, 52, 61, 54, 19, 22, 36, 29, 5,
    ],
    //7:
    [
        35, 40, 59, 49, 9, 42, 58, 17, 56, 34, 8, 2, 57, 24, 51, 50, 33, 18, 48, 26, 25, 16, 43, 1,
        11, 53, 60, 12, 30, 29, 5, 19, 61, 46, 44, 22, 54, 14, 27, 21, 36, 45, 38, 3, 6, 20, 13,
        52,
    ],
    //8:
    [
        56, 32, 51, 41, 1, 34, 50, 9, 48, 26, 0, 59, 49, 16, 43, 42, 25, 10, 40, 18, 17, 8, 35, 58,
        3, 45, 52, 4, 22, 21, 60, 11, 53, 38, 36, 14, 46, 6, 19, 13, 28, 37, 30, 62, 61, 12, 5, 44,
    ],
    //9:
    [
        40, 16, 35, 25, 50, 18, 34, 58, 32, 10, 49, 43, 33, 0, 56, 26, 9, 59, 24, 2, 1, 57, 48, 42,
        54, 29, 36, 19, 6, 5, 44, 62, 37, 22, 20, 61, 30, 53, 3, 60, 12, 21, 14, 46, 45, 27, 52,
        28,
    ],
    //10:
    [
        24, 0, 48, 9, 34, 2, 18, 42, 16, 59, 33, 56, 17, 49, 40, 10, 58, 43, 8, 51, 50, 41, 32, 26,
        38, 13, 20, 3, 53, 52, 28, 46, 21, 6, 4, 45, 14, 37, 54, 44, 27, 5, 61, 30, 29, 11, 36, 12,
    ],
    //11:
    [
        8, 49, 32, 58, 18, 51, 2, 26, 0, 43, 17, 40, 1, 33, 24, 59, 42, 56, 57, 35, 34, 25, 16, 10,
        22, 60, 4, 54, 37, 36, 12, 30, 5, 53, 19, 29, 61, 21, 38, 28, 11, 52, 45, 14, 13, 62, 20,
        27,
    ],
    //12:
    [
        57, 33, 16, 42, 2, 35, 51, 10, 49, 56, 1, 24, 50, 17, 8, 43, 26, 40, 41, 48, 18, 9, 0, 59,
        6, 44, 19, 38, 21, 20, 27, 14, 52, 37, 3, 13, 45, 5, 22, 12, 62, 36, 29, 61, 60, 46, 4, 11,
    ],
    //13:
    [
        41, 17, 0, 26, 51, 48, 35, 59, 33, 40, 50, 8, 34, 1, 57, 56, 10, 24, 25, 32, 2, 58, 49, 43,
        53, 28, 3, 22, 5, 4, 11, 61, 36, 21, 54, 60, 29, 52, 6, 27, 46, 20, 13, 45, 44, 30, 19, 62,
    ],
    //14:
    [
        25, 1, 49, 10, 35, 32, 48, 43, 17, 24, 34, 57, 18, 50, 41, 40, 59, 8, 9, 16, 51, 42, 33,
        56, 37, 12, 54, 6, 52, 19, 62, 45, 20, 5, 38, 44, 13, 36, 53, 11, 30, 4, 60, 29, 28, 14, 3,
        46,
    ],
    //15:
    [
        17, 58, 41, 2, 56, 24, 40, 35, 9, 16, 26, 49, 10, 42, 33, 32, 51, 0, 1, 8, 43, 34, 25, 48,
        29, 4, 46, 61, 44, 11, 54, 37, 12, 60, 30, 36, 5, 28, 45, 3, 22, 27, 52, 21, 20, 6, 62, 38,
    ],
];

pub fn encrypt_simd(plaintext: u64, keys: &mut [u64x8; 64]) {
    let ip = permute_bits_pc(&IP, plaintext, 64);
    let mut l: [u64x8; 32] = [ZERO; 32];
    let mut r: [u64x8; 32] = [ZERO; 32];
    for i in 0..32 {
        let bit_l = (ip >> (63 - i)) & 1;
        let bit_r = (ip >> (31 - i)) & 1;

        l[i] = u64x8::splat(0u64.wrapping_sub(bit_l));
        r[i] = u64x8::splat(0u64.wrapping_sub(bit_r));
    }

    // round 0
    unsafe {
        feistel_function_simd(&mut l, &mut r, keys, 0);
    }
    // round 1
    unsafe {
        feistel_function_simd(&mut r, &mut l, keys, 1);
    }
    // round 2
    unsafe {
        feistel_function_simd(&mut l, &mut r, keys, 2);
    }
    // round 3
    unsafe {
        feistel_function_simd(&mut r, &mut l, keys, 3);
    }
    // round 4
    unsafe {
        feistel_function_simd(&mut l, &mut r, keys, 4);
    }
    // round 5
    unsafe {
        feistel_function_simd(&mut r, &mut l, keys, 5);
    }
    // round 6
    unsafe {
        feistel_function_simd(&mut l, &mut r, keys, 6);
    }
    // round 7
    unsafe {
        feistel_function_simd(&mut r, &mut l, keys, 7);
    }
    // round 8
    unsafe {
        feistel_function_simd(&mut l, &mut r, keys, 8);
    }
    // round 9
    unsafe {
        feistel_function_simd(&mut r, &mut l, keys, 9);
    }
    // round 10
    unsafe {
        feistel_function_simd(&mut l, &mut r, keys, 10);
    }
    // round 11
    unsafe {
        feistel_function_simd(&mut r, &mut l, keys, 11);
    }
    // round 12
    unsafe {
        feistel_function_simd(&mut l, &mut r, keys, 12);
    }
    // round 13
    unsafe {
        feistel_function_simd(&mut r, &mut l, keys, 13);
    }
    // round 14
    unsafe {
        feistel_function_simd(&mut l, &mut r, keys, 14);
    }
    // round 15
    unsafe {
        feistel_function_simd(&mut r, &mut l, keys, 15);
    }

    //final permutation
    let keys_ptr = keys.as_mut_ptr();
    for i in 0..64 {
        unsafe {
            std::ptr::write(
                keys_ptr.add(i),
                if IP_INVO[i] < 32 {
                    r[IP_INVO[i]]
                } else {
                    l[IP_INVO[i] - 32]
                },
            );
        }
    }
}

#[inline(always)]
pub unsafe fn feistel_function_simd(
    l: &mut [u64x8; 32],
    r: &mut [u64x8; 32],
    keys: &[u64x8; 64],
    round: usize,
) {
    //s1 expand
    let mut e0 = r[31] ^ keys[SUBKEY_SCHEDULE[round][0]];
    let mut e1 = r[0] ^ keys[SUBKEY_SCHEDULE[round][1]];
    let mut e2 = r[1] ^ keys[SUBKEY_SCHEDULE[round][2]];
    let mut e3 = r[2] ^ keys[SUBKEY_SCHEDULE[round][3]];
    let mut e4 = r[3] ^ keys[SUBKEY_SCHEDULE[round][4]];
    let mut e5 = r[4] ^ keys[SUBKEY_SCHEDULE[round][5]];

    //s2 expand
    let mut f0 = r[3] ^ keys[SUBKEY_SCHEDULE[round][6]];
    let mut f1 = r[4] ^ keys[SUBKEY_SCHEDULE[round][7]];
    let mut f2 = r[5] ^ keys[SUBKEY_SCHEDULE[round][8]];
    let mut f3 = r[6] ^ keys[SUBKEY_SCHEDULE[round][9]];
    let mut f4 = r[7] ^ keys[SUBKEY_SCHEDULE[round][10]];
    let mut f5 = r[8] ^ keys[SUBKEY_SCHEDULE[round][11]];

    //s3 expand
    let mut g0 = r[7] ^ keys[SUBKEY_SCHEDULE[round][12]];
    let mut g1 = r[8] ^ keys[SUBKEY_SCHEDULE[round][13]];
    let mut g2 = r[9] ^ keys[SUBKEY_SCHEDULE[round][14]];
    let mut g3 = r[10] ^ keys[SUBKEY_SCHEDULE[round][15]];
    let mut g4 = r[11] ^ keys[SUBKEY_SCHEDULE[round][16]];
    let mut g5 = r[12] ^ keys[SUBKEY_SCHEDULE[round][17]];

    //s4 expand
    let mut h0 = r[11] ^ keys[SUBKEY_SCHEDULE[round][18]];
    let mut h1 = r[12] ^ keys[SUBKEY_SCHEDULE[round][19]];
    let mut h2 = r[13] ^ keys[SUBKEY_SCHEDULE[round][20]];
    let mut h3 = r[14] ^ keys[SUBKEY_SCHEDULE[round][21]];
    let mut h4 = r[15] ^ keys[SUBKEY_SCHEDULE[round][22]];
    let mut h5 = r[16] ^ keys[SUBKEY_SCHEDULE[round][23]];

    //s1 compute
    //use inner block to free o registers
    {
        let (o0, o1, o2, o3) = s1(e0, e1, e2, e3, e4, e5);
        l[8] = l[8] ^ o0;
        l[16] = l[16] ^ o1;
        l[22] = l[22] ^ o2;
        l[30] = l[30] ^ o3;
    }

    //s2 compute
    {
        let (o0, o1, o2, o3) = s2(f0, f1, f2, f3, f4, f5);
        l[12] = l[12] ^ o0;
        l[27] = l[27] ^ o1;
        l[1] = l[1] ^ o2;
        l[17] = l[17] ^ o3;
    }

    //s3 compute
    {
        let (o0, o1, o2, o3) = s3(g0, g1, g2, g3, g4, g5);
        l[23] = l[23] ^ o0;
        l[15] = l[15] ^ o1;
        l[29] = l[29] ^ o2;
        l[5] = l[5] ^ o3;
    }

    //s4 compute
    {
        let (o0, o1, o2, o3) = s4(h0, h1, h2, h3, h4, h5);
        l[25] = l[25] ^ o0;
        l[19] = l[19] ^ o1;
        l[9] = l[9] ^ o2;
        l[0] = l[0] ^ o3;
    }

    //s5 expand
    e0 = r[15] ^ keys[SUBKEY_SCHEDULE[round][24]];
    e1 = r[16] ^ keys[SUBKEY_SCHEDULE[round][25]];
    e2 = r[17] ^ keys[SUBKEY_SCHEDULE[round][26]];
    e3 = r[18] ^ keys[SUBKEY_SCHEDULE[round][27]];
    e4 = r[19] ^ keys[SUBKEY_SCHEDULE[round][28]];
    e5 = r[20] ^ keys[SUBKEY_SCHEDULE[round][29]];

    //s6 expand
    f0 = r[19] ^ keys[SUBKEY_SCHEDULE[round][30]];
    f1 = r[20] ^ keys[SUBKEY_SCHEDULE[round][31]];
    f2 = r[21] ^ keys[SUBKEY_SCHEDULE[round][32]];
    f3 = r[22] ^ keys[SUBKEY_SCHEDULE[round][33]];
    f4 = r[23] ^ keys[SUBKEY_SCHEDULE[round][34]];
    f5 = r[24] ^ keys[SUBKEY_SCHEDULE[round][35]];

    //s7 expand
    g0 = r[23] ^ keys[SUBKEY_SCHEDULE[round][36]];
    g1 = r[24] ^ keys[SUBKEY_SCHEDULE[round][37]];
    g2 = r[25] ^ keys[SUBKEY_SCHEDULE[round][38]];
    g3 = r[26] ^ keys[SUBKEY_SCHEDULE[round][39]];
    g4 = r[27] ^ keys[SUBKEY_SCHEDULE[round][40]];
    g5 = r[28] ^ keys[SUBKEY_SCHEDULE[round][41]];

    //s8 expand
    h0 = r[27] ^ keys[SUBKEY_SCHEDULE[round][42]];
    h1 = r[28] ^ keys[SUBKEY_SCHEDULE[round][43]];
    h2 = r[29] ^ keys[SUBKEY_SCHEDULE[round][44]];
    h3 = r[30] ^ keys[SUBKEY_SCHEDULE[round][45]];
    h4 = r[31] ^ keys[SUBKEY_SCHEDULE[round][46]];
    h5 = r[0] ^ keys[SUBKEY_SCHEDULE[round][47]];

    //s5 compute
    {
        let (o0, o1, o2, o3) = s5(e0, e1, e2, e3, e4, e5);
        l[7] = l[7] ^ o0;
        l[13] = l[13] ^ o1;
        l[24] = l[24] ^ o2;
        l[2] = l[2] ^ o3;
    }

    //s6 compute
    {
        let (o0, o1, o2, o3) = s6(f0, f1, f2, f3, f4, f5);
        l[3] = l[3] ^ o0;
        l[28] = l[28] ^ o1;
        l[10] = l[10] ^ o2;
        l[18] = l[18] ^ o3;
    }

    //s7 compute
    {
        let (o0, o1, o2, o3) = s7(g0, g1, g2, g3, g4, g5);
        l[31] = l[31] ^ o0;
        l[11] = l[11] ^ o1;
        l[21] = l[21] ^ o2;
        l[6] = l[6] ^ o3;
    }

    //s8 compute
    {
        let (o0, o1, o2, o3) = s8(h0, h1, h2, h3, h4, h5);
        l[4] = l[4] ^ o0;
        l[26] = l[26] ^ o1;
        l[14] = l[14] ^ o2;
        l[20] = l[20] ^ o3;
    }
}

const ALL_ONES_AVX: __m512i = unsafe { transmute([!0u64; 8]) };
const ZERO_AVX: __m512i = unsafe { transmute([0u64; 8]) };

#[inline(always)]
pub fn encrypt_avx_512(keys: &mut [__m512i; 64]) {
    //assume pre calculated l and r for IP
    //TODO: round 0 can simply read from 2 registers (zero's or all ones), the rounds after need to work with l and r fully
    let mut l: [__m512i; 32] = [
        ZERO_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
    ];
    let mut r: [__m512i; 32] = [
        ALL_ONES_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
        ZERO_AVX,
        ALL_ONES_AVX,
        ALL_ONES_AVX,
        ZERO_AVX,
    ];
    // round 0
    unsafe {
        feistel_avx_512(&mut l, &mut r, keys, 0);
    }
    // round 1
    unsafe {
        feistel_avx_512(&mut r, &mut l, keys, 1);
    }
    // round 2
    unsafe {
        feistel_avx_512(&mut l, &mut r, keys, 2);
    }
    // round 3
    unsafe {
        feistel_avx_512(&mut r, &mut l, keys, 3);
    }
    // round 4
    unsafe {
        feistel_avx_512(&mut l, &mut r, keys, 4);
    }
    // round 5
    unsafe {
        feistel_avx_512(&mut r, &mut l, keys, 5);
    }
    // round 6
    unsafe {
        feistel_avx_512(&mut l, &mut r, keys, 6);
    }
    // round 7
    unsafe {
        feistel_avx_512(&mut r, &mut l, keys, 7);
    }
    // round 8
    unsafe {
        feistel_avx_512(&mut l, &mut r, keys, 8);
    }
    // round 9
    unsafe {
        feistel_avx_512(&mut r, &mut l, keys, 9);
    }
    // round 10
    unsafe {
        feistel_avx_512(&mut l, &mut r, keys, 10);
    }
    // round 11
    unsafe {
        feistel_avx_512(&mut r, &mut l, keys, 11);
    }
    // round 12
    unsafe {
        feistel_avx_512(&mut l, &mut r, keys, 12);
    }
    // round 13
    unsafe {
        feistel_avx_512(&mut r, &mut l, keys, 13);
    }
    // round 14
    unsafe {
        feistel_avx_512(&mut l, &mut r, keys, 14);
    }
    // round 15
    unsafe {
        feistel_avx_512(&mut r, &mut l, keys, 15);
    }

    //final permutation
    unsafe {
        let out = keys.as_mut_ptr();

        *out.add(0) = l[7];
        *out.add(1) = r[7];
        *out.add(2) = l[15];
        *out.add(3) = r[15];
        *out.add(4) = l[23];
        *out.add(5) = r[23];
        *out.add(6) = l[31];
        *out.add(7) = r[31];

        *out.add(8) = l[6];
        *out.add(9) = r[6];
        *out.add(10) = l[14];
        *out.add(11) = r[14];
        *out.add(12) = l[22];
        *out.add(13) = r[22];
        *out.add(14) = l[30];
        *out.add(15) = r[30];

        *out.add(16) = l[5];
        *out.add(17) = r[5];
        *out.add(18) = l[13];
        *out.add(19) = r[13];
        *out.add(20) = l[21];
        *out.add(21) = r[21];
        *out.add(22) = l[29];
        *out.add(23) = r[29];

        *out.add(24) = l[4];
        *out.add(25) = r[4];
        *out.add(26) = l[12];
        *out.add(27) = r[12];
        *out.add(28) = l[20];
        *out.add(29) = r[20];
        *out.add(30) = l[28];
        *out.add(31) = r[28];

        *out.add(32) = l[3];
        *out.add(33) = r[3];
        *out.add(34) = l[11];
        *out.add(35) = r[11];
        *out.add(36) = l[19];
        *out.add(37) = r[19];
        *out.add(38) = l[27];
        *out.add(39) = r[27];

        *out.add(40) = l[2];
        *out.add(41) = r[2];
        *out.add(42) = l[10];
        *out.add(43) = r[10];
        *out.add(44) = l[18];
        *out.add(45) = r[18];
        *out.add(46) = l[26];
        *out.add(47) = r[26];

        *out.add(48) = l[1];
        *out.add(49) = r[1];
        *out.add(50) = l[9];
        *out.add(51) = r[9];
        *out.add(52) = l[17];
        *out.add(53) = r[17];
        *out.add(54) = l[25];
        *out.add(55) = r[25];

        *out.add(56) = l[0];
        *out.add(57) = r[0];
        *out.add(58) = l[8];
        *out.add(59) = r[8];
        *out.add(60) = l[16];
        *out.add(61) = r[16];
        *out.add(62) = l[24];
        *out.add(63) = r[24];
    }
}

#[inline(always)]
pub unsafe fn feistel_avx_512(
    l: &mut [__m512i; 32],
    r: &mut [__m512i; 32],
    keys: &[__m512i; 64],
    round: usize,
) {
    unsafe {
        //s1 expand
        let mut e0 = _mm512_xor_si512(r[31], keys[SUBKEY_SCHEDULE[round][0]]);
        let mut e1 = _mm512_xor_si512(r[0], keys[SUBKEY_SCHEDULE[round][1]]);
        let mut e2 = _mm512_xor_si512(r[1], keys[SUBKEY_SCHEDULE[round][2]]);
        let mut e3 = _mm512_xor_si512(r[2], keys[SUBKEY_SCHEDULE[round][3]]);
        let mut e4 = _mm512_xor_si512(r[3], keys[SUBKEY_SCHEDULE[round][4]]);
        let mut e5 = _mm512_xor_si512(r[4], keys[SUBKEY_SCHEDULE[round][5]]);

        //s2 expand
        let mut f0 = _mm512_xor_si512(r[3], keys[SUBKEY_SCHEDULE[round][6]]);
        let mut f1 = _mm512_xor_si512(r[4], keys[SUBKEY_SCHEDULE[round][7]]);
        let mut f2 = _mm512_xor_si512(r[5], keys[SUBKEY_SCHEDULE[round][8]]);
        let mut f3 = _mm512_xor_si512(r[6], keys[SUBKEY_SCHEDULE[round][9]]);
        let mut f4 = _mm512_xor_si512(r[7], keys[SUBKEY_SCHEDULE[round][10]]);
        let mut f5 = _mm512_xor_si512(r[8], keys[SUBKEY_SCHEDULE[round][11]]);

        //s3 expand
        let mut g0 = _mm512_xor_si512(r[7], keys[SUBKEY_SCHEDULE[round][12]]);
        let mut g1 = _mm512_xor_si512(r[8], keys[SUBKEY_SCHEDULE[round][13]]);
        let mut g2 = _mm512_xor_si512(r[9], keys[SUBKEY_SCHEDULE[round][14]]);
        let mut g3 = _mm512_xor_si512(r[10], keys[SUBKEY_SCHEDULE[round][15]]);
        let mut g4 = _mm512_xor_si512(r[11], keys[SUBKEY_SCHEDULE[round][16]]);
        let mut g5 = _mm512_xor_si512(r[12], keys[SUBKEY_SCHEDULE[round][17]]);

        //s4 expand
        let mut h0 = _mm512_xor_si512(r[11], keys[SUBKEY_SCHEDULE[round][18]]);
        let mut h1 = _mm512_xor_si512(r[12], keys[SUBKEY_SCHEDULE[round][19]]);
        let mut h2 = _mm512_xor_si512(r[13], keys[SUBKEY_SCHEDULE[round][20]]);
        let mut h3 = _mm512_xor_si512(r[14], keys[SUBKEY_SCHEDULE[round][21]]);
        let mut h4 = _mm512_xor_si512(r[15], keys[SUBKEY_SCHEDULE[round][22]]);
        let mut h5 = _mm512_xor_si512(r[16], keys[SUBKEY_SCHEDULE[round][23]]);

        //s1 compute
        //use inner block to free o registers
        {
            let (o0, o1, o2, o3) = s1_avx_512(e0, e1, e2, e3, e4, e5);
            l[8] = _mm512_xor_si512(l[8], o0);
            l[16] = _mm512_xor_si512(l[16], o1);
            l[22] = _mm512_xor_si512(l[22], o2);
            l[30] = _mm512_xor_si512(l[30], o3);
        }

        //s2 compute
        {
            let (o0, o1, o2, o3) = s2_avx_512(f0, f1, f2, f3, f4, f5);
            l[12] = _mm512_xor_si512(l[12], o0);
            l[27] = _mm512_xor_si512(l[27], o1);
            l[1] = _mm512_xor_si512(l[1], o2);
            l[17] = _mm512_xor_si512(l[17], o3);
        }

        //s3 compute
        {
            let (o0, o1, o2, o3) = s3_avx_512(g0, g1, g2, g3, g4, g5);
            l[23] = _mm512_xor_si512(l[23], o0);
            l[15] = _mm512_xor_si512(l[15], o1);
            l[29] = _mm512_xor_si512(l[29], o2);
            l[5] = _mm512_xor_si512(l[5], o3);
        }

        //s4 compute
        {
            let (o0, o1, o2, o3) = s4_avx_512(h0, h1, h2, h3, h4, h5);
            l[25] = _mm512_xor_si512(l[25], o0);
            l[19] = _mm512_xor_si512(l[19], o1);
            l[9] = _mm512_xor_si512(l[9], o2);
            l[0] = _mm512_xor_si512(l[0], o3);
        }

        //s5 expand
        e0 = _mm512_xor_si512(r[15], keys[SUBKEY_SCHEDULE[round][24]]);
        e1 = _mm512_xor_si512(r[16], keys[SUBKEY_SCHEDULE[round][25]]);
        e2 = _mm512_xor_si512(r[17], keys[SUBKEY_SCHEDULE[round][26]]);
        e3 = _mm512_xor_si512(r[18], keys[SUBKEY_SCHEDULE[round][27]]);
        e4 = _mm512_xor_si512(r[19], keys[SUBKEY_SCHEDULE[round][28]]);
        e5 = _mm512_xor_si512(r[20], keys[SUBKEY_SCHEDULE[round][29]]);

        //s6 expand
        f0 = _mm512_xor_si512(r[19], keys[SUBKEY_SCHEDULE[round][30]]);
        f1 = _mm512_xor_si512(r[20], keys[SUBKEY_SCHEDULE[round][31]]);
        f2 = _mm512_xor_si512(r[21], keys[SUBKEY_SCHEDULE[round][32]]);
        f3 = _mm512_xor_si512(r[22], keys[SUBKEY_SCHEDULE[round][33]]);
        f4 = _mm512_xor_si512(r[23], keys[SUBKEY_SCHEDULE[round][34]]);
        f5 = _mm512_xor_si512(r[24], keys[SUBKEY_SCHEDULE[round][35]]);

        //s7 expand
        g0 = _mm512_xor_si512(r[23], keys[SUBKEY_SCHEDULE[round][36]]);
        g1 = _mm512_xor_si512(r[24], keys[SUBKEY_SCHEDULE[round][37]]);
        g2 = _mm512_xor_si512(r[25], keys[SUBKEY_SCHEDULE[round][38]]);
        g3 = _mm512_xor_si512(r[26], keys[SUBKEY_SCHEDULE[round][39]]);
        g4 = _mm512_xor_si512(r[27], keys[SUBKEY_SCHEDULE[round][40]]);
        g5 = _mm512_xor_si512(r[28], keys[SUBKEY_SCHEDULE[round][41]]);

        //s8 expand
        h0 = _mm512_xor_si512(r[27], keys[SUBKEY_SCHEDULE[round][42]]);
        h1 = _mm512_xor_si512(r[28], keys[SUBKEY_SCHEDULE[round][43]]);
        h2 = _mm512_xor_si512(r[29], keys[SUBKEY_SCHEDULE[round][44]]);
        h3 = _mm512_xor_si512(r[30], keys[SUBKEY_SCHEDULE[round][45]]);
        h4 = _mm512_xor_si512(r[31], keys[SUBKEY_SCHEDULE[round][46]]);
        h5 = _mm512_xor_si512(r[0], keys[SUBKEY_SCHEDULE[round][47]]);

        //s5 compute
        {
            let (o0, o1, o2, o3) = s5_avx_512(e0, e1, e2, e3, e4, e5);
            l[7] = _mm512_xor_si512(l[7], o0);
            l[13] = _mm512_xor_si512(l[13], o1);
            l[24] = _mm512_xor_si512(l[24], o2);
            l[2] = _mm512_xor_si512(l[2], o3);
        }

        //s6 compute
        {
            let (o0, o1, o2, o3) = s6_avx_512(f0, f1, f2, f3, f4, f5);
            l[3] = _mm512_xor_si512(l[3], o0);
            l[28] = _mm512_xor_si512(l[28], o1);
            l[10] = _mm512_xor_si512(l[10], o2);
            l[18] = _mm512_xor_si512(l[18], o3);
        }

        //s7 compute
        {
            let (o0, o1, o2, o3) = s7_avx_512(g0, g1, g2, g3, g4, g5);
            l[31] = _mm512_xor_si512(l[31], o0);
            l[11] = _mm512_xor_si512(l[11], o1);
            l[21] = _mm512_xor_si512(l[21], o2);
            l[6] = _mm512_xor_si512(l[6], o3);
        }

        //s8 compute
        {
            let (o0, o1, o2, o3) = s8_avx_512(h0, h1, h2, h3, h4, h5);
            l[4] = _mm512_xor_si512(l[4], o0);
            l[26] = _mm512_xor_si512(l[26], o1);
            l[14] = _mm512_xor_si512(l[14], o2);
            l[20] = _mm512_xor_si512(l[20], o3);
        }
    }
}

//AVX 2
const ALL_ONES_AVX_2: __m256i = unsafe { transmute([!0u64; 4]) };
const ZERO_AVX_2: __m256i = unsafe { transmute([0u64; 4]) };

#[inline(always)]
pub fn encrypt_avx_2(keys: &mut [__m256i; 64]) {
    //assume pre calculated l and r for IP
    //TODO: round 0 can simply read from 2 registers (zero's or all ones), the rounds after need to work with l and r fully
    let mut l: [__m256i; 32] = [
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
    ];
    let mut r: [__m256i; 32] = [
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
        ZERO_AVX_2,
        ALL_ONES_AVX_2,
        ALL_ONES_AVX_2,
        ZERO_AVX_2,
    ];
    // round 0
    unsafe {
        feistel_avx_2(&mut l, &mut r, keys, 0);
    }
    // round 1
    unsafe {
        feistel_avx_2(&mut r, &mut l, keys, 1);
    }
    // round 2
    unsafe {
        feistel_avx_2(&mut l, &mut r, keys, 2);
    }
    // round 3
    unsafe {
        feistel_avx_2(&mut r, &mut l, keys, 3);
    }
    // round 4
    unsafe {
        feistel_avx_2(&mut l, &mut r, keys, 4);
    }
    // round 5
    unsafe {
        feistel_avx_2(&mut r, &mut l, keys, 5);
    }
    // round 6
    unsafe {
        feistel_avx_2(&mut l, &mut r, keys, 6);
    }
    // round 7
    unsafe {
        feistel_avx_2(&mut r, &mut l, keys, 7);
    }
    // round 8
    unsafe {
        feistel_avx_2(&mut l, &mut r, keys, 8);
    }
    // round 9
    unsafe {
        feistel_avx_2(&mut r, &mut l, keys, 9);
    }
    // round 10
    unsafe {
        feistel_avx_2(&mut l, &mut r, keys, 10);
    }
    // round 11
    unsafe {
        feistel_avx_2(&mut r, &mut l, keys, 11);
    }
    // round 12
    unsafe {
        feistel_avx_2(&mut l, &mut r, keys, 12);
    }
    // round 13
    unsafe {
        feistel_avx_2(&mut r, &mut l, keys, 13);
    }
    // round 14
    unsafe {
        feistel_avx_2(&mut l, &mut r, keys, 14);
    }
    // round 15
    unsafe {
        feistel_avx_2(&mut r, &mut l, keys, 15);
    }

    //final permutation
    unsafe {
        let out = keys.as_mut_ptr();

        *out.add(0) = l[7];
        *out.add(1) = r[7];
        *out.add(2) = l[15];
        *out.add(3) = r[15];
        *out.add(4) = l[23];
        *out.add(5) = r[23];
        *out.add(6) = l[31];
        *out.add(7) = r[31];

        *out.add(8) = l[6];
        *out.add(9) = r[6];
        *out.add(10) = l[14];
        *out.add(11) = r[14];
        *out.add(12) = l[22];
        *out.add(13) = r[22];
        *out.add(14) = l[30];
        *out.add(15) = r[30];

        *out.add(16) = l[5];
        *out.add(17) = r[5];
        *out.add(18) = l[13];
        *out.add(19) = r[13];
        *out.add(20) = l[21];
        *out.add(21) = r[21];
        *out.add(22) = l[29];
        *out.add(23) = r[29];

        *out.add(24) = l[4];
        *out.add(25) = r[4];
        *out.add(26) = l[12];
        *out.add(27) = r[12];
        *out.add(28) = l[20];
        *out.add(29) = r[20];
        *out.add(30) = l[28];
        *out.add(31) = r[28];

        *out.add(32) = l[3];
        *out.add(33) = r[3];
        *out.add(34) = l[11];
        *out.add(35) = r[11];
        *out.add(36) = l[19];
        *out.add(37) = r[19];
        *out.add(38) = l[27];
        *out.add(39) = r[27];

        *out.add(40) = l[2];
        *out.add(41) = r[2];
        *out.add(42) = l[10];
        *out.add(43) = r[10];
        *out.add(44) = l[18];
        *out.add(45) = r[18];
        *out.add(46) = l[26];
        *out.add(47) = r[26];

        *out.add(48) = l[1];
        *out.add(49) = r[1];
        *out.add(50) = l[9];
        *out.add(51) = r[9];
        *out.add(52) = l[17];
        *out.add(53) = r[17];
        *out.add(54) = l[25];
        *out.add(55) = r[25];

        *out.add(56) = l[0];
        *out.add(57) = r[0];
        *out.add(58) = l[8];
        *out.add(59) = r[8];
        *out.add(60) = l[16];
        *out.add(61) = r[16];
        *out.add(62) = l[24];
        *out.add(63) = r[24];
    }
}

#[inline(always)]
pub unsafe fn feistel_avx_2(
    l: &mut [__m256i; 32],
    r: &mut [__m256i; 32],
    keys: &[__m256i; 64],
    round: usize,
) {
    unsafe {
        //s1 expand
        let mut e0 = _mm256_xor_si256(r[31], keys[SUBKEY_SCHEDULE[round][0]]);
        let mut e1 = _mm256_xor_si256(r[0], keys[SUBKEY_SCHEDULE[round][1]]);
        let mut e2 = _mm256_xor_si256(r[1], keys[SUBKEY_SCHEDULE[round][2]]);
        let mut e3 = _mm256_xor_si256(r[2], keys[SUBKEY_SCHEDULE[round][3]]);
        let mut e4 = _mm256_xor_si256(r[3], keys[SUBKEY_SCHEDULE[round][4]]);
        let mut e5 = _mm256_xor_si256(r[4], keys[SUBKEY_SCHEDULE[round][5]]);

        //s2 expand
        let mut f0 = _mm256_xor_si256(r[3], keys[SUBKEY_SCHEDULE[round][6]]);
        let mut f1 = _mm256_xor_si256(r[4], keys[SUBKEY_SCHEDULE[round][7]]);
        let mut f2 = _mm256_xor_si256(r[5], keys[SUBKEY_SCHEDULE[round][8]]);
        let mut f3 = _mm256_xor_si256(r[6], keys[SUBKEY_SCHEDULE[round][9]]);
        let mut f4 = _mm256_xor_si256(r[7], keys[SUBKEY_SCHEDULE[round][10]]);
        let mut f5 = _mm256_xor_si256(r[8], keys[SUBKEY_SCHEDULE[round][11]]);

        //s3 expand
        let mut g0 = _mm256_xor_si256(r[7], keys[SUBKEY_SCHEDULE[round][12]]);
        let mut g1 = _mm256_xor_si256(r[8], keys[SUBKEY_SCHEDULE[round][13]]);
        let mut g2 = _mm256_xor_si256(r[9], keys[SUBKEY_SCHEDULE[round][14]]);
        let mut g3 = _mm256_xor_si256(r[10], keys[SUBKEY_SCHEDULE[round][15]]);
        let mut g4 = _mm256_xor_si256(r[11], keys[SUBKEY_SCHEDULE[round][16]]);
        let mut g5 = _mm256_xor_si256(r[12], keys[SUBKEY_SCHEDULE[round][17]]);

        //s4 expand
        let mut h0 = _mm256_xor_si256(r[11], keys[SUBKEY_SCHEDULE[round][18]]);
        let mut h1 = _mm256_xor_si256(r[12], keys[SUBKEY_SCHEDULE[round][19]]);
        let mut h2 = _mm256_xor_si256(r[13], keys[SUBKEY_SCHEDULE[round][20]]);
        let mut h3 = _mm256_xor_si256(r[14], keys[SUBKEY_SCHEDULE[round][21]]);
        let mut h4 = _mm256_xor_si256(r[15], keys[SUBKEY_SCHEDULE[round][22]]);
        let mut h5 = _mm256_xor_si256(r[16], keys[SUBKEY_SCHEDULE[round][23]]);

        //s1 compute
        //use inner block to free o registers
        {
            let (o0, o1, o2, o3) = s1_avx_2(e0, e1, e2, e3, e4, e5);
            l[8] = _mm256_xor_si256(l[8], o0);
            l[16] = _mm256_xor_si256(l[16], o1);
            l[22] = _mm256_xor_si256(l[22], o2);
            l[30] = _mm256_xor_si256(l[30], o3);
        }

        //s2 compute
        {
            let (o0, o1, o2, o3) = s2_avx_2(f0, f1, f2, f3, f4, f5);
            l[12] = _mm256_xor_si256(l[12], o0);
            l[27] = _mm256_xor_si256(l[27], o1);
            l[1] = _mm256_xor_si256(l[1], o2);
            l[17] = _mm256_xor_si256(l[17], o3);
        }

        //s3 compute
        {
            let (o0, o1, o2, o3) = s3_avx_2(g0, g1, g2, g3, g4, g5);
            l[23] = _mm256_xor_si256(l[23], o0);
            l[15] = _mm256_xor_si256(l[15], o1);
            l[29] = _mm256_xor_si256(l[29], o2);
            l[5] = _mm256_xor_si256(l[5], o3);
        }

        //s4 compute
        {
            let (o0, o1, o2, o3) = s4_avx_2(h0, h1, h2, h3, h4, h5);
            l[25] = _mm256_xor_si256(l[25], o0);
            l[19] = _mm256_xor_si256(l[19], o1);
            l[9] = _mm256_xor_si256(l[9], o2);
            l[0] = _mm256_xor_si256(l[0], o3);
        }

        //s5 expand
        e0 = _mm256_xor_si256(r[15], keys[SUBKEY_SCHEDULE[round][24]]);
        e1 = _mm256_xor_si256(r[16], keys[SUBKEY_SCHEDULE[round][25]]);
        e2 = _mm256_xor_si256(r[17], keys[SUBKEY_SCHEDULE[round][26]]);
        e3 = _mm256_xor_si256(r[18], keys[SUBKEY_SCHEDULE[round][27]]);
        e4 = _mm256_xor_si256(r[19], keys[SUBKEY_SCHEDULE[round][28]]);
        e5 = _mm256_xor_si256(r[20], keys[SUBKEY_SCHEDULE[round][29]]);

        //s6 expand
        f0 = _mm256_xor_si256(r[19], keys[SUBKEY_SCHEDULE[round][30]]);
        f1 = _mm256_xor_si256(r[20], keys[SUBKEY_SCHEDULE[round][31]]);
        f2 = _mm256_xor_si256(r[21], keys[SUBKEY_SCHEDULE[round][32]]);
        f3 = _mm256_xor_si256(r[22], keys[SUBKEY_SCHEDULE[round][33]]);
        f4 = _mm256_xor_si256(r[23], keys[SUBKEY_SCHEDULE[round][34]]);
        f5 = _mm256_xor_si256(r[24], keys[SUBKEY_SCHEDULE[round][35]]);

        //s7 expand
        g0 = _mm256_xor_si256(r[23], keys[SUBKEY_SCHEDULE[round][36]]);
        g1 = _mm256_xor_si256(r[24], keys[SUBKEY_SCHEDULE[round][37]]);
        g2 = _mm256_xor_si256(r[25], keys[SUBKEY_SCHEDULE[round][38]]);
        g3 = _mm256_xor_si256(r[26], keys[SUBKEY_SCHEDULE[round][39]]);
        g4 = _mm256_xor_si256(r[27], keys[SUBKEY_SCHEDULE[round][40]]);
        g5 = _mm256_xor_si256(r[28], keys[SUBKEY_SCHEDULE[round][41]]);

        //s8 expand
        h0 = _mm256_xor_si256(r[27], keys[SUBKEY_SCHEDULE[round][42]]);
        h1 = _mm256_xor_si256(r[28], keys[SUBKEY_SCHEDULE[round][43]]);
        h2 = _mm256_xor_si256(r[29], keys[SUBKEY_SCHEDULE[round][44]]);
        h3 = _mm256_xor_si256(r[30], keys[SUBKEY_SCHEDULE[round][45]]);
        h4 = _mm256_xor_si256(r[31], keys[SUBKEY_SCHEDULE[round][46]]);
        h5 = _mm256_xor_si256(r[0], keys[SUBKEY_SCHEDULE[round][47]]);

        //s5 compute
        {
            let (o0, o1, o2, o3) = s5_avx_2(e0, e1, e2, e3, e4, e5);
            l[7] = _mm256_xor_si256(l[7], o0);
            l[13] = _mm256_xor_si256(l[13], o1);
            l[24] = _mm256_xor_si256(l[24], o2);
            l[2] = _mm256_xor_si256(l[2], o3);
        }

        //s6 compute
        {
            let (o0, o1, o2, o3) = s6_avx_2(f0, f1, f2, f3, f4, f5);
            l[3] = _mm256_xor_si256(l[3], o0);
            l[28] = _mm256_xor_si256(l[28], o1);
            l[10] = _mm256_xor_si256(l[10], o2);
            l[18] = _mm256_xor_si256(l[18], o3);
        }

        //s7 compute
        {
            let (o0, o1, o2, o3) = s7_avx_2(g0, g1, g2, g3, g4, g5);
            l[31] = _mm256_xor_si256(l[31], o0);
            l[11] = _mm256_xor_si256(l[11], o1);
            l[21] = _mm256_xor_si256(l[21], o2);
            l[6] = _mm256_xor_si256(l[6], o3);
        }

        //s8 compute
        {
            let (o0, o1, o2, o3) = s8_avx_2(h0, h1, h2, h3, h4, h5);
            l[4] = _mm256_xor_si256(l[4], o0);
            l[26] = _mm256_xor_si256(l[26], o1);
            l[14] = _mm256_xor_si256(l[14], o2);
            l[20] = _mm256_xor_si256(l[20], o3);
        }
    }
}

fn permute_bits_pc(p_box: &[u64], input: u64, input_length: usize) -> u64 {
    let p_box_length = p_box.len();
    let mut output: u64 = 0;
    for i in 0..p_box_length {
        output |= permute_bits::<u64>(input, input_length, p_box[i] as usize, p_box_length, i + 1);
    }
    output
}

fn permute_bits<T>(
    input: T,
    input_length: usize,
    input_index: usize,
    output_length: usize,
    output_index: usize,
) -> T
where
    T: Copy
        + Shl<usize, Output = T>
        + Shr<usize, Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + From<u64>,
{
    ((input >> (input_length - input_index)) & T::from(1)) << (output_length - output_index)
}
