use crate::Args;

mod bench_1_1;
mod util;

// use self::bench_1_1::*;

pub fn run_benchmark(suite: &str, _args: &Args) {
    match suite {
        // "1_1_a" => bench_1_1_a(args),
        // "1_1_b" => bench_1_1_b(args),
        // "1_1_c" => bench_1_1_c(args),
        // "1_1_d" => bench_1_1_d(args),
        // "1_1_e" => bench_1_1_e(args),
        // "1_1_f" => bench_1_1_f(args),
        _ => {
            println!("Could not find the benchmark");
        }
    }
}
