use crate::GeneticArgs;

mod bench_balls;
mod bench_heart;
mod util;

use self::bench_balls::*;
use self::bench_heart::*;

pub fn run_benchmark(suite: &str, args: &GeneticArgs) {
    match suite {
        "small_ball" => bench_small_ball(args),
        "big_ball" => bench_big_ball(args),
        "<3" | "heart" => bench_heart(args),
        // "1_1_d" => bench_1_1_d(args),
        // "1_1_e" => bench_1_1_e(args),
        // "1_1_f" => bench_1_1_f(args),
        _ => {
            println!("Could not find the benchmark");
        }
    }
}
