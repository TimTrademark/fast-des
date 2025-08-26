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
            benchmark("normal_des", 1_000_000, 10000, || {
                des(PLAINTEXT, KEY);
            });
        }
        "f" | "fast" | "fast_des" => {
            let keys: [u64; 64] = vec![0xABCDEF1234567890u64; 64].try_into().unwrap();
            benchmark("fast_des", 1_000_000, 10000, || {
                des_optimized(PLAINTEXT, keys);
            });
        }
        _ => {
            println!("Invalid benchmark name")
        }
    }
}
