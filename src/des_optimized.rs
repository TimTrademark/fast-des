use std::ops::{BitAnd, BitOr, Shl, Shr};

use crate::{
    ZERO,
    constants::{EO, IP, IP_INVO, PC_1O, PC_2O, PO},
    sbox_optimized::{s1, s2, s3, s4, s5, s6, s7, s8},
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
    //repeat and transpote IP
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
