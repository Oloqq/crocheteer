use super::util::execute_benchmark;
use crate::genetic::fitness_funcs::*;
use crate::genetic::params::{Case, GrowingParams, Params};
use crate::Args;

// 1.1.A Program powinien wygenerować na wyjściu (na dowolnej pozycji w danych wyjściowych) liczbę 1. Poza liczbą 1 może też zwrócić inne liczby.
pub fn bench_1_1_a(args: &Args) {
    let params = Params {
        memsize: 3,
        popsize: 100,
        max_size: 10,
        p_crossover: 0.9,
        p_mut_per_node: 0.2,
        tournament_size: 2,
        random_initial_memory: true,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.2,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![1]),
        (vec![1], vec![1]),
        (vec![1, 2], vec![1]),
    ];

    execute_benchmark(args, params, cases, "1_1_a", diff_best);
}

// 1.1.B Program powinien wygenerować na wyjściu (na dowolnej pozycji w danych wyjściowych) liczbę 789. Poza liczbą 789 może też zwrócić inne liczby.
pub fn bench_1_1_b(args: &Args) {
    let params = Params {
        memsize: 3,
        popsize: 1000,
        max_size: 10, // ignored during initial generation, low number prevents bloating
        p_crossover: 0.9,
        p_mut_per_node: 0.1,
        tournament_size: 2,
        acceptable_error: -1.0,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.1,
            max_const: 1000,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![789]),
        (vec![1], vec![789]),
        (vec![1, 2], vec![789]),
    ];

    execute_benchmark(args, params, cases, "1_1_b", diff_best);
}

// 1.1.C Program powinien wygenerować na wyjściu (na dowolnej pozycji w danych wyjściowych) liczbę 31415. Poza liczbą 31415 może też zwrócić inne liczby.
pub fn bench_1_1_c(args: &Args) {
    let params = Params {
        memsize: 3,
        popsize: 2000,
        max_size: 10,
        p_crossover: 0.5,
        p_mut_per_node: 0.1,
        tournament_size: 2,
        acceptable_error: -10.0,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.1,
            max_const: 100000,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![31415]),
        (vec![1], vec![31415]),
        (vec![1, 2], vec![31415]),
    ];

    execute_benchmark(args, params, cases, "1_1_c", diff_best);
}

// 1.1.D Program powinien wygenerować na pierwszej pozycji na wyjściu liczbę 1. Poza liczbą 1 może też zwrócić inne liczby.
pub fn bench_1_1_d(args: &Args) {
    let params = Params {
        memsize: 3,
        popsize: 100,
        max_size: 10,
        p_crossover: 0.9,
        p_mut_per_node: 0.2,
        tournament_size: 2,
        random_initial_memory: true,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.2,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![1]),
        (vec![1], vec![1]),
        (vec![1, 2], vec![1]),
    ];

    execute_benchmark(args, params, cases, "1_1_d", diff_first);
}

// 1.1.E Program powinien wygenerować na pierwszej pozycji na wyjściu liczbę 789. Poza liczbą 789 może też zwrócić inne liczby.
pub fn bench_1_1_e(args: &Args) {
    let params = Params {
        memsize: 3,
        popsize: 1000,
        max_size: 10, // ignored during initial generation, low number prevents bloating
        p_crossover: 0.9,
        p_mut_per_node: 0.1,
        tournament_size: 2,
        acceptable_error: -1.0,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.1,
            max_const: 1000,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![789]),
        (vec![1], vec![789]),
        (vec![1, 2], vec![789]),
    ];

    execute_benchmark(args, params, cases, "1_1_e", diff_first);
}

// 1.1.F Program powinien wygenerować na wyjściu liczbę jako jedyną liczbę 1. Poza liczbą 1 NIE powinien nic więcej wygenerować.
pub fn bench_1_1_f(args: &Args) {
    let params = Params {
        memsize: 10,
        popsize: 1000,
        max_size: 4,
        p_crossover: 0.2,
        p_mut_per_node: 0.5,
        tournament_size: 2,
        random_initial_memory: true,
        acceptable_error: -1e-3,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.2,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![1]),
        (vec![1], vec![1]),
        (vec![3, 2], vec![1]),
        (vec![5, 2], vec![1]),
        (vec![6, 4], vec![1]),
        (vec![8, 5], vec![1]),
        (vec![865, 6], vec![1]),
    ];

    execute_benchmark(args, params, cases, "1_1_f", diff_only);
}
