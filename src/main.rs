use std::{
    arch::x86_64::{__m256i, __m512i, _mm256_setzero_si256, _mm512_setzero_si512},
    env,
    sync::Arc,
    thread,
};

use fast_des::{
    benchmark::{benchmark, benchmark_parallel},
    bitsliced_des_inline_simd, bitsliced_des_inline_simd_avx_512, bitsliced_des_simd,
    bitsliced_des_simd_avx_512, bitsliced_netntlmv1_inline_simd_avx_2,
    bitsliced_netntlmv1_inline_simd_avx_512, bitsliced_netntlmv1_simd, des,
};

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
        "davx512" | "des_avx_512" => unsafe {
            let keys = [[0; 64]; 8];

            benchmark("des_avx_512", 10_000_000, 1_000_000, 512, || {
                bitsliced_des_simd_avx_512(&keys);
            });
        },
        "davx512i" | "des_avx_512_inline" => unsafe {
            let mut keys: [__m512i; 64] = [_mm512_setzero_si512(); 64];

            benchmark("des_avx_512_inline", 10_000_000, 1_000_000, 512, || {
                bitsliced_des_inline_simd_avx_512(&mut keys);
            });
        },
        "davx512ip" | "des_avx_512_parallel" => unsafe {
            let keys: [__m512i; 64] = [_mm512_setzero_si512(); 64];
            benchmark_parallel("des_avx_512_parallel", 10_000_000, 512, 32, move || {
                let mut keys = keys.clone();
                bitsliced_des_inline_simd_avx_512(&mut keys);
            });
        },
        "navx512i" | "netntlmv1_avx_512_inline" => unsafe {
            let mut keys: [__m512i; 64] = [_mm512_setzero_si512(); 64];

            benchmark(
                "netntlmv1_avx_512_inline",
                10_000_000,
                1_000_000,
                512,
                || {
                    bitsliced_netntlmv1_inline_simd_avx_512(&mut keys);
                },
            );
        },
        "navxi512p" | "netntlmv1_avx_512_parallel" => unsafe {
            let keys: [__m512i; 64] = [_mm512_setzero_si512(); 64];
            benchmark_parallel(
                "netntlmv1_avx_512_parallel",
                10_000_000,
                512,
                32,
                move || {
                    let mut keys = keys.clone();
                    bitsliced_netntlmv1_inline_simd_avx_512(&mut keys);
                },
            );
        },
        "navx2i" | "netntlmv1_avx_2_inline" => unsafe {
            let mut keys: [__m256i; 64] = [_mm256_setzero_si256(); 64];

            benchmark("netntlmv1_avx_2_inline", 10_000_000, 1_000_000, 512, || {
                bitsliced_netntlmv1_inline_simd_avx_2(&mut keys);
            });
        },
        "navx2ip" | "netntlmv1_avx_2_parallel" => unsafe {
            let keys: [__m256i; 64] = [_mm256_setzero_si256(); 64];
            benchmark_parallel("netntlmv1_avx_2_parallel", 10_000_000, 512, 16, move || {
                let mut keys = keys.clone();
                bitsliced_netntlmv1_inline_simd_avx_2(&mut keys);
            });
        },

        _ => {
            println!("Invalid benchmark name")
        }
    }
}
