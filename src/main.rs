use std::{cell::RefCell, env};

use ::des::{
    Des,
    cipher::{BlockEncrypt, KeyInit, consts::U8, generic_array::GenericArray},
};
use fast_des::{benchmark::benchmark, des, des_optimized};

const KEY: u64 = 0xABCDEF1234567890;
const PLAINTEXT: u64 = 0x8877665544332211;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: fast-des.exe <benchmark_name>");
    }
    let benchmark_name = &args[1];
    start_benchmark(benchmark_name.as_str());
}

fn start_benchmark(benchmark_name: &str) {
    match benchmark_name {
        "n" | "normal" | "normal_des" => {
            benchmark("normal_des", 100_000, 10000, 1, || {
                des(PLAINTEXT, KEY);
            });
        }
        "f" | "fast" | "fast_des" => {
            let keys = [0x133457799BBCDFF1u64; 64];

            benchmark("fast_des", 100_000, 10000, 64, || {
                des_optimized(PLAINTEXT, &keys);
            });
        }
        "d" => {
            let v: u64 = 0x133457799BBCDFF1;
            let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(v) };
            let mut bytesarray: GenericArray<u8, U8> =
                GenericArray::clone_from_slice(&raw_bytes[0..8]);
            bytesarray.reverse();

            benchmark("des", 100_000, 10000, 1, || {
                let mut cipher = GenericArray::clone(&bytesarray);
                let hasher = Des::new(&bytesarray);
                hasher.encrypt_block(&mut cipher);
            });
        }
        _ => {
            println!("Invalid benchmark name")
        }
    }
}
