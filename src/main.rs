use std::env;

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
            let keys = [[0x133457799BBCDFF1u64; 64]; 8];

            benchmark("fast_des", 100_000, 10000, 64 * 8, || {
                des_optimized::<8>(PLAINTEXT, &keys);
            });
        }
        _ => {
            println!("Invalid benchmark name")
        }
    }
}
