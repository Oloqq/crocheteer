use std::io::{self, Write};

use crate::{params::{Params, Case, GrowingParams}, tinygp::{diff_best, fitness_funcs::diff_first}};

use super::{
    common::*,
    // evolution::*,
    TinyGP
};

#[test]
#[ignore]
fn test_e2e_identity() {
    let params = Params {
        seed: 0,
        memsize: 3,
        popsize: 100,
        max_size: 4,
        p_crossover: 0.9,
        p_mut_per_node: 0.05,
        tournament_size: 2,
        acceptable_error: 0.1,
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![1], vec![1]),
        (vec![4], vec![4]),
        (vec![10], vec![10])
    ];
    let writer: Box<dyn Write> = Box::new(io::stdout());
    let seed = Some(0);
    let mut tgp = TinyGP::new(params, cases, seed, writer.into(), diff_first);
    let (program, fitness) = tgp.evolve(3, diff_first);
    println!("{:?}", tgp.population);
    println!("{:?}", tgp.fitness);
    println!("{:?}", program);
    assert_eq!(fitness, 0.0);
}

#[test]
#[ignore]
fn test_e2e_gen_1() { // 1.1.A, 1.1.D, 1.1.F
    let params = Params {
        seed: 0,
        memsize: 3,
        popsize: 100,
        max_size: 4,
        p_crossover: 0.9,
        p_mut_per_node: 0.05,
        tournament_size: 2,
        acceptable_error: -0.1,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.2,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![1]),
        (vec![1], vec![1]),
        (vec![1, 2], vec![1])
    ];
    let writer: Box<dyn Write> = Box::new(io::stdout());
    let seed = Some(0);
    let mut tgp = TinyGP::new(params, cases, seed, writer.into(), diff_best);
    let (program, fitness) = tgp.evolve(3, diff_best);
    println!("{:?}", program);
    println!("{:?}", fitness);
    assert_eq!(fitness, 0.0);
}

#[test]
#[ignore]
fn test_e2e_gen_789() { // 1.1.B, 1.1.E, 1.1.C analogicznie
    let params = Params {
        seed: 0,
        memsize: 3,
        popsize: 100,
        max_size: 4,
        acceptable_error: 0.0,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.0,
            min_const: 789,
            max_const: 790,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![], vec![789]),
        (vec![], vec![789]),
        (vec![], vec![789])
    ];
    let writer: Box<dyn Write> = Box::new(io::stdout());
    let seed = Some(0);
    let mut tgp = TinyGP::new(params, cases, seed, writer.into(), diff_first);
    let (program, fitness) = tgp.evolve(3, diff_first);
    println!("{:?}", program);
    println!("{:?}", fitness);
    assert_eq!(fitness, 0.0);
}

#[test]
#[ignore]
fn test_e2e_sum() { // 1.2.B, 1.2C
    const ONLY_VARIANT_MATTERS_I32: i32 = 0;
    const ONLY_VARIANT_MATTERS_USIZE: usize = 0;
    let params = Params {
        seed: 0,
        memsize: 3,
        popsize: 1000,
        max_size: 4,
        acceptable_error: 0.0,
        growing: GrowingParams {
            p_prefer_reg_over_num: 1.0,
            p_expression_plug: 0.5,
            d_expr: vec![
                (Expr::ADD, 1),
                (Expr::SUB, 0),
                (Expr::MUL, 0),
                (Expr::DIV, 0),
                (Expr::EQ, 0),
                (Expr::LT, 0),
                (Expr::GT, 0),
                (Expr::OR, 0),
                (Expr::AND, 0),
                (Expr::NOT, 0),
                (Expr::Num(ONLY_VARIANT_MATTERS_I32), 1),
                (Expr::Reg(ONLY_VARIANT_MATTERS_USIZE), 1),
            ],
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![-9, 9], vec![0]),
        (vec![0, 4], vec![4]),
        (vec![1, 2], vec![3]),
        (vec![-9999, 9999], vec![0]),
    ];
    let writer: Box<dyn Write> = Box::new(io::stdout());
    let seed = Some(0);
    let mut tgp = TinyGP::new(params, cases, seed, writer.into(), diff_first);
    let (program, fitness) = tgp.evolve(3, diff_first);
    println!("{:?}", program);
    println!("{:?}", fitness);
    assert_eq!(fitness, 0.0);
}


#[test]
#[ignore]
fn test_e2e_diff() {
    const ONLY_VARIANT_MATTERS_I32: i32 = 0;
    const ONLY_VARIANT_MATTERS_USIZE: usize = 0;
    let params = Params {
        seed: 0,
        memsize: 3,
        popsize: 1000,
        max_size: 4,
        acceptable_error: 0.0,
        growing: GrowingParams {
            p_prefer_reg_over_num: 1.0,
            p_expression_plug: 0.5,
            d_expr: vec![
                (Expr::ADD, 0),
                (Expr::SUB, 1),
                (Expr::MUL, 0),
                (Expr::DIV, 0),
                (Expr::EQ, 0),
                (Expr::LT, 0),
                (Expr::GT, 0),
                (Expr::OR, 0),
                (Expr::AND, 0),
                (Expr::NOT, 0),
                (Expr::Num(ONLY_VARIANT_MATTERS_I32), 1),
                (Expr::Reg(ONLY_VARIANT_MATTERS_USIZE), 1),
            ],
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![-9, 9], vec![-18]),
        (vec![0, 4], vec![-4]),
        (vec![1, 2], vec![-1]),
        (vec![-9999, 9999], vec![-9999 - 9999]),
    ];
    let writer: Box<dyn Write> = Box::new(io::stdout());
    let seed = Some(0);
    let mut tgp = TinyGP::new(params, cases, seed, writer.into(), diff_first);
    let (program, fitness) = tgp.evolve(3, diff_first);
    println!("{:?}", program);
    println!("{:?}", fitness);
    assert_eq!(fitness, 0.0);
}