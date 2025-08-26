use std::ops::{BitAnd, BitOr, Shl, Shr};

use crate::{
    constants::{E, EO, IP, IP_INV, IP_INVO, IPO, P, PC_1, PC_1O, PC_2, PC_2O, PO, S_BOXES},
    sbox_optimized::{s1, s2, s3, s4, s5, s6, s7, s8},
};

const C_D_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111;

// outputs 56 bit k+
pub fn compute_pc1(k: u64) -> (u32, u32) {
    let k_plus = permute_bits_pc(&PC_1, k, 64);
    let d0 = (k_plus & C_D_MASK) as u32;
    let c0 = ((k_plus >> 28) & C_D_MASK) as u32;
    (c0, d0)
}

//slice[i] contains bit with index i of all 64 slices
// a column is equal to one item/number
pub fn compute_pc1_optimized(k_slices: [u64; 64]) -> ([u64; 28], [u64; 28]) {
    let mut c0 = [0u64; 28];
    let mut d0 = [0u64; 28];

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

pub fn create_subkeys(c0: u32, d0: u32) -> [u64; 16] {
    let mut subkeys = [0u64; 16];
    let mut c = c0;
    let mut d = d0;

    for (index, s) in SHIFTS.iter().enumerate() {
        c = rotl28(c, *s);
        d = rotl28(d, *s);

        let cd: u64 = (0u64 | d as u64) | ((c as u64) << 28);
        let k = permute_bits_pc(&PC_2, cd, 56);
        subkeys[index] = k;
    }
    subkeys
}
//TODO: consider passing C0 and d0 as mut directly
pub fn create_subkeys_optimized(c0: [u64; 28], d0: [u64; 28]) -> [[u64; 48]; 16] {
    let mut subkeys = [[0u64; 48]; 16];
    let mut c = c0;
    let mut d = d0;

    for (index, s) in SHIFTS.iter().enumerate() {
        c.rotate_left(*s as usize);
        d.rotate_left(*s as usize);
        let mut cd = [0u64; 56];
        cd[..28].copy_from_slice(&c);
        cd[28..].copy_from_slice(&d);

        let mut k = [0u64; 48];
        for (i, &src) in PC_2O.iter().enumerate() {
            k[i] = cd[src as usize];
        }
        subkeys[index] = k;
    }

    subkeys
}

const U32_MASK: u32 = 0xFFFFFFFF;

pub fn encrypt(plaintext: u64, subkeys: [u64; 16]) -> u64 {
    let ip = permute_bits_pc(&IP, plaintext, 64);
    let mut l = (ip >> 32) as u32 & U32_MASK;
    let mut r = ip as u32 & U32_MASK;
    for i in 0..16 {
        let (new_l, new_r) = feistel_function(l, r, subkeys[i]);
        l = new_l;
        r = new_r;
    }
    let r_l = ((r as u64) << 32) | l as u64;
    let fp = permute_bits_pc(&IP_INV, r_l, 64);
    fp
}
//TODO: create a version where IP is precomputed
pub fn encrypt_optimized(plaintext: u64, subkeys: [[u64; 48]; 16]) -> [u64; 64] {
    let ip = permute_bits_pc(&IP, plaintext, 64);
    //repeat and transpote IP
    let ip_bitsliced = transpose_u64_to_bitsliced(&vec![ip; 64]);
    let mut l: [u64; 32] = ip_bitsliced[0..32].try_into().unwrap();
    let mut r: [u64; 32] = ip_bitsliced[32..64].try_into().unwrap();
    for i in 0..16 {
        let (new_l, new_r) = feistel_function_optimized(l, r, subkeys[i]);
        l = new_l;
        r = new_r;
    }
    let mut r_l = [0u64; 64];
    r_l[..32].copy_from_slice(&r);
    r_l[32..].copy_from_slice(&l);
    let mut fp = [0u64; 64];
    for (i, &src) in IP_INVO.iter().enumerate() {
        fp[i] = r_l[src as usize];
    }
    fp
}

const SIX_BIT_MASK: u8 = 0b111111;

fn feistel_function(l: u32, r: u32, subkey: u64) -> (u32, u32) {
    let new_l = r;
    let e = permute_bits_pc(&E, r as u64, 32);
    let e1 = e ^ subkey;
    let mut output: u32 = 0;
    for i in 0..8 {
        let offset = 48 - (6 * (i + 1));
        let b = (e1 >> offset) as u8 & SIX_BIT_MASK;
        let s = s_box_lookup(i, b);
        let output_offset: u32 = (32 - (4 * (i + 1))) as u32;
        output |= (s as u32) << output_offset;
    }
    output = permute_bits_pc(&P, output as u64, 32) as u32;
    let new_r = l ^ output;
    (new_l, new_r)
}

fn feistel_function_optimized(
    l: [u64; 32],
    r: [u64; 32],
    subkey: [u64; 48],
) -> ([u64; 32], [u64; 32]) {
    let new_l = r;
    let mut e = [0u64; 48];
    for (i, &src) in EO.iter().enumerate() {
        e[i] = r[src as usize];
    }
    for i in 0..48 {
        e[i] ^= subkey[i];
    }
    let mut output: [u64; 32] = [0u64; 32];
    // s1
    let s1_output = s1(e[0], e[1], e[2], e[3], e[4], e[5]);
    output[0] = s1_output.0;
    output[1] = s1_output.1;
    output[2] = s1_output.2;
    output[3] = s1_output.3;

    // s2
    let s2_output = s2(e[6], e[7], e[8], e[9], e[10], e[11]);
    output[4] = s2_output.0;
    output[5] = s2_output.1;
    output[6] = s2_output.2;
    output[7] = s2_output.3;

    // s3
    let s3_output = s3(e[12], e[13], e[14], e[15], e[16], e[17]);
    output[8] = s3_output.0;
    output[9] = s3_output.1;
    output[10] = s3_output.2;
    output[11] = s3_output.3;

    // s4
    let s4_output = s4(e[18], e[19], e[20], e[21], e[22], e[23]);
    output[12] = s4_output.0;
    output[13] = s4_output.1;
    output[14] = s4_output.2;
    output[15] = s4_output.3;

    // s5
    let s5_output = s5(e[24], e[25], e[26], e[27], e[28], e[29]);
    output[16] = s5_output.0;
    output[17] = s5_output.1;
    output[18] = s5_output.2;
    output[19] = s5_output.3;

    // s6
    let s6_output = s6(e[30], e[31], e[32], e[33], e[34], e[35]);
    output[20] = s6_output.0;
    output[21] = s6_output.1;
    output[22] = s6_output.2;
    output[23] = s6_output.3;

    // s7
    let s7_output = s7(e[36], e[37], e[38], e[39], e[40], e[41]);
    output[24] = s7_output.0;
    output[25] = s7_output.1;
    output[26] = s7_output.2;
    output[27] = s7_output.3;

    // s8
    let s8_output = s8(e[42], e[43], e[44], e[45], e[46], e[47]);
    output[28] = s8_output.0;
    output[29] = s8_output.1;
    output[30] = s8_output.2;
    output[31] = s8_output.3;
    let mut new_r = [0u64; 32];
    for (i, &src) in PO.iter().enumerate() {
        new_r[i] = output[src as usize];
    }
    //inline array modification to avoid extra allocation
    for i in 0..32 {
        new_r[i] = l[i] ^ new_r[i];
    }
    (new_l, new_r)
}

fn s_box_lookup(sbox: usize, input6: u8) -> u8 {
    let row = ((input6 & 0b100000) >> 4) | (input6 & 0b000001);
    let col = (input6 & 0b011110) >> 1;
    S_BOXES[sbox][row as usize][col as usize]
}

const ROTL28_MASK: u32 = (1 << 28) - 1;

fn rotl28(x: u32, n: u32) -> u32 {
    ((x << n) | (x >> (28 - n))) & ROTL28_MASK
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

//TODO: replace with an optimized version
pub fn transpose_bitsliced_to_u64(rows: &[u64]) -> [u64; 64] {
    let mut cols = [0u64; 64];
    let rows_len = rows.len();
    for (bitpos, &row) in rows.iter().enumerate() {
        for col in 0..64 {
            let bit = (row >> col) & 1;
            cols[col] |= bit << rows_len - 1 - bitpos;
        }
    }
    cols
}

pub fn transpose_u64_to_bitsliced(numbers: &[u64]) -> [u64; 64] {
    let mut rows = [0u64; 64];
    let mut row_index = 0;
    let rows_len = rows.len();
    while row_index < rows_len {
        for i in 0..rows_len {
            //get bit with index row_index for number
            let bit = (numbers[i] >> (rows_len - 1 - row_index)) & 0b1;
            //set bit at column i in row with row index row_index
            rows[row_index] |= bit << (rows_len - 1 - i);
        }
        row_index += 1;
    }
    rows
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_subkeys_optimized_works() {
        //as this is a test, we're only populating the first column
        let c0: [u64; 28] = [
            1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1,
        ];
        let d0: [u64; 28] = [
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1,
        ];

        let subkeys = create_subkeys_optimized(c0, d0);

        let transposed = transpose_bitsliced_to_u64(&subkeys[0]);
        let k0 = transposed[0];

        let transposed = transpose_bitsliced_to_u64(&subkeys[15]);
        let k15 = transposed[0];
        dbg!(format!("{:b}", k0));
        assert_eq!(k0, 0b000110110000001011101111111111000111000001110010);
        assert_eq!(k15, 0b110010110011110110001011000011100001011111110101);
    }

    #[test]
    fn test_transpose_int_to_bitsliced_works() {
        let number: u64 = 0b1011;
        let numbers = vec![number; 64];
        let output = transpose_u64_to_bitsliced(&numbers);
        assert_eq!(output[60], 0xFFFFFFFFFFFFFFFF);
        assert_eq!(output[61], 0);
        assert_eq!(output[62], 0xFFFFFFFFFFFFFFFF);
        assert_eq!(output[63], 0xFFFFFFFFFFFFFFFF);
    }
}
