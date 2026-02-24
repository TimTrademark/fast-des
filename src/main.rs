use std::{
    arch::x86_64::{__m512i, _mm512_setzero_si512},
    env,
    sync::Arc,
    thread,
};

use fast_des::{
    benchmark::{benchmark, benchmark_parallel},
    bitsliced_des_inline_simd, bitsliced_des_inline_simd_avx, bitsliced_des_simd,
    bitsliced_des_simd_avx, bitsliced_netntlmv1_inline_simd_avx, bitsliced_netntlmv1_simd, des,
};
use wide::u64x8;

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
        "f" | "fast" | "fast_des_simd" => {
            let mut keys = [[0x133457799BBCDFF1u64; 64]; 8];

            benchmark("fast_des_simd", 1_000_000, 10000, 64 * 8, || {
                bitsliced_des_simd(PLAINTEXT, &mut keys);
            });
        }
        "davx" | "des_avx" => unsafe {
            let keys = [[0; 64]; 8];

            benchmark("des_avx", 10_000_000, 1_000_000, 512, || {
                bitsliced_des_simd_avx(&keys);
            });
        },
        "davxi" | "des_avx_inline" => unsafe {
            let mut keys: [__m512i; 64] = [_mm512_setzero_si512(); 64];

            benchmark("des_avx_inline", 10_000_000, 1_000_000, 512, || {
                bitsliced_des_inline_simd_avx(&mut keys);
            });
        },
        "davxip" | "des_avx_parallel" => unsafe {
            let keys: [__m512i; 64] = [_mm512_setzero_si512(); 64];
            benchmark_parallel("des_avx_parallel", 10_000_000, 512, 32, move || {
                let mut keys = keys.clone();
                bitsliced_des_inline_simd_avx(&mut keys);
            });
        },
        "navxi" | "netntlmv1_avx_inline" => unsafe {
            let mut keys: [__m512i; 64] = [_mm512_setzero_si512(); 64];

            benchmark("netntlmv1_avx_inline", 10_000_000, 1_000_000, 512, || {
                bitsliced_netntlmv1_inline_simd_avx(&mut keys);
            });
        },
        "navxip" | "netntlmv1_avx_parallel" => unsafe {
            let keys: [__m512i; 64] = [_mm512_setzero_si512(); 64];
            benchmark_parallel("netntlmv1_avx_parallel", 10_000_000, 512, 32, move || {
                let mut keys = keys.clone();
                bitsliced_netntlmv1_inline_simd_avx(&mut keys);
            });
        },

        _ => {
            println!("Invalid benchmark name")
        }
    }
}
