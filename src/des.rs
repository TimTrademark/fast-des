use std::ops::{BitAnd, BitOr, Shl, Shr};

use crate::{
    constants::{E, IP, IP_INV, P, PC_1, PC_2, S_BOXES},
    utils::{print_binary_u32, print_binary_u64},
};

const C_D_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111;

// outputs 56 bit k+
pub fn compute_pc1(k: u64) -> (u32, u32) {
    let k_plus = permute_bits_pc(&PC_1, k, 64);
    let d0 = (k_plus & C_D_MASK) as u32;
    let c0 = ((k_plus >> 28) & C_D_MASK) as u32;
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_subkeys_works() {
        let subkeys = create_subkeys(
            0b1111000011001100101010101111,
            0b0101010101100110011110001111,
        );
        assert_eq!(
            subkeys[0],
            0b000110110000001011101111111111000111000001110010
        );
        assert_eq!(
            subkeys[15],
            0b110010110011110110001011000011100001011111110101
        );
    }

    #[test]
    fn test_feistel_function_works() {
        let output = feistel_function(
            0b11001100000000001100110011111111,
            0b11110000101010101111000010101010,
            0b000110110000001011101111111111000111000001110010,
        );
        assert_eq!(output.0, 0b11110000101010101111000010101010);
        assert_eq!(output.1, 0b11101111010010100110010101000100);
    }
}
