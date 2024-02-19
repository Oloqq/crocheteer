use super::{
    common::*,
    execution::{execute, Runtime},
};

const INPUT: Token = Token::Stat(Stat::INPUT);
const OUTPUT: Token = Token::Stat(Stat::OUTPUT);
const LOAD: Token = Token::Stat(Stat::LOAD);
const IF: Token = Token::Stat(Stat::IF);
const WHILE: Token = Token::Stat(Stat::WHILE);
const ELSE: Token = Token::ELSE;
const END: Token = Token::END;
use Token::Reg;

fn num(x: Number) -> Token {
    Token::Expr(Expr::Num(x))
}

use pretty_assertions::assert_eq;

fn run_cases(program: &Program, memsize: usize, cases: Vec<(Vec<Number>, Vec<Number>)>) {
    let _ = env_logger::builder().is_test(true).try_init();
    for (i, (input, expected_output)) in cases.into_iter().enumerate() {
        let mut runtime = Runtime::new(memsize, &input, &mut None);
        println!("\nCase {i}");
        execute(&program, &mut runtime);
        assert_eq!(runtime.output, expected_output);
    }
}

#[test]
fn test_identity() {
    let memsize = 3;
    let program = vec![INPUT, Token::Reg(0), OUTPUT, Token::Reg(0)];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![(vec![1], vec![1]), (vec![2], vec![2])];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_load_register() {
    let memsize = 2;
    let program = vec![
        INPUT, Reg(0),
        LOAD, Reg(1), Reg(0),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![1], vec![1, 1]),
        (vec![2], vec![2, 2])
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_load_expr_add() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::ADD), Reg(0), Reg(1),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
        OUTPUT, Reg(2),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![1, 2], vec![1, 2, 3]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_load_expr_sub() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::SUB), Reg(0), Reg(1),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
        OUTPUT, Reg(2),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![1, 2], vec![1, 2, -1]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_load_expr_mul() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::MUL), Reg(0), Reg(1),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
        OUTPUT, Reg(2),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![2, 2], vec![2, 2, 4]),
    ];
    run_cases(&program, memsize, cases);
}


#[test]
#[rustfmt::skip]
fn test_load_expr_protected_div() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::DIV), Reg(0), Reg(1),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
        OUTPUT, Reg(2),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![1, 2], vec![1, 2, 0]),
        (vec![1, 0], vec![1, 0, 1]),
        (vec![6, 2], vec![6, 2, 3]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_output_num_literal() {
    let memsize = 3;
    let program = vec![
        OUTPUT, Token::Expr(Expr::Num(21))
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![], vec![21]),
        (vec![1], vec![21]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_expr_with_literal() {
    let memsize = 3;
    let program = vec![
        OUTPUT, Token::Expr(Expr::ADD),
            Token::Expr(Expr::Num(2)),
            Token::Expr(Expr::Num(1))
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![], vec![3]),
        (vec![1], vec![3]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_if_true() {
    let memsize = 3;
    let program = vec![
        OUTPUT, num(1),
        IF, num(1),
            OUTPUT, num(2),
        END,
        OUTPUT, num(3),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![], vec![1, 2, 3]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_if_false() {
    let memsize = 3;
    let program = vec![
        OUTPUT, num(1),
        IF, num(0),
            OUTPUT, num(2),
        END,
        OUTPUT, num(3),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![], vec![1, 3]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_if_nested() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        OUTPUT, num(1),
        IF, Reg(0),
            OUTPUT, num(2),
            IF, Reg(1),
                OUTPUT, num(3),
            END,
        END,
        OUTPUT, num(4),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![0, 0], vec![1, 4]),
        (vec![0, 1], vec![1, 4]),
        (vec![1, 0], vec![1, 2, 4]),
        (vec![1, 1], vec![1, 2, 3, 4]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_if_else_nested() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        OUTPUT, num(1),
        IF, Reg(0), // 6
            OUTPUT, num(2),
            IF, Reg(1), // 10
                OUTPUT, num(3),
            ELSE, // 14
                OUTPUT, num(5),
                WHILE, num(0),
                    OUTPUT, num(997),
                END,
            END,
        ELSE, // 18
            OUTPUT, num(6),
        END, // 21
        OUTPUT, num(4),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![0, 0], vec![1, 6, 4]),
        (vec![0, 1], vec![1, 6, 4]),
        (vec![1, 0], vec![1, 2, 5, 4]),
        (vec![1, 1], vec![1, 2, 3, 4]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_while_false() {
    let memsize = 3;
    let program = vec![
        OUTPUT, num(1),
        WHILE, num(0),
            OUTPUT, num(2),
        END,
        OUTPUT, num(3)
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![], vec![1, 3]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_while_interrupt_infinite_loop() {
    let memsize = 3;
    let program = vec![
        OUTPUT, num(1),
        WHILE, num(1),
            OUTPUT, num(2),
        END,
        OUTPUT, num(3)
    ];
    let v = vec![];
    let mut runtime = Runtime::new(memsize, &v, &mut None);
    println!("Entering infinite loop");
    execute(&program, &mut runtime);
    let output = runtime.output;
    assert!(output.len() > 3);
    assert_eq!(output[0], 1);
    assert_eq!(output[1], 2);
    assert_eq!(output[2], 2);
}

#[test]
#[rustfmt::skip]
fn test_while_condition_change() {
    let memsize = 3;
    let program = vec![
        OUTPUT, num(1),
        LOAD, Reg(0), num(1),
        WHILE, Reg(0),
            OUTPUT, num(2),
            LOAD, Reg(0), num(0),
        END,
        OUTPUT, num(3)
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![], vec![1, 2, 3]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_while_nested() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        OUTPUT, num(1),
        WHILE, Reg(0),
            LOAD, Reg(0), num(0),
            OUTPUT, num(2),
            WHILE, Reg(1),
                LOAD, Reg(1), num(0),
                OUTPUT, num(3),
            END,
            OUTPUT, num(19),
        END,
        OUTPUT, num(4),
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![0, 0], vec![1, 4]),
        (vec![0, 1], vec![1, 4]),
        (vec![1, 0], vec![1, 2, 19, 4]),
        (vec![1, 1], vec![1, 2, 3, 19, 4]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_while_many_iterations() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        WHILE, Reg(0),
            OUTPUT, Reg(0),
            LOAD, Reg(0), Token::Expr(Expr::SUB),
                Reg(0), num(1),
        END,
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![0], vec![]),
        (vec![1], vec![1]),
        (vec![2], vec![2, 1]),
        (vec![3], vec![3, 2, 1]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_expr_eq() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::EQ), Reg(0), Reg(1),
        OUTPUT, Reg(2)
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![1, 2], vec![0]),
        (vec![-1, -1], vec![1]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_expr_less_than() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::LT), Reg(0), Reg(1),
        OUTPUT, Reg(2)
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![1, 2], vec![1]),
        (vec![-1, -2], vec![0]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_expr_greater_than() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::GT), Reg(0), Reg(1),
        OUTPUT, Reg(2)
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![1, 2], vec![0]),
        (vec![-1, -2], vec![1]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_expr_or() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::OR), Reg(0), Reg(1),
        OUTPUT, Reg(2)
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![0, 0], vec![0]),
        (vec![0, 1], vec![1]),
        (vec![1, 0], vec![1]),
        (vec![1, 1], vec![1]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_expr_and() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::AND), Reg(0), Reg(1),
        OUTPUT, Reg(2)
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![0, 0], vec![0]),
        (vec![0, 1], vec![0]),
        (vec![1, 0], vec![0]),
        (vec![1, 1], vec![1]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_expr_not() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        LOAD, Reg(2),
            Token::Expr(Expr::NOT), Reg(0),
        OUTPUT, Reg(2)
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![0], vec![1]),
        (vec![1], vec![0]),
    ];
    run_cases(&program, memsize, cases);
}


#[test]
#[rustfmt::skip]
#[ignore]
fn test_while_faulty() {
    let memsize = 3;
    const ADD: Token = Token::Expr(Expr::ADD);
    let program = vec![
        INPUT, Reg(0),
        WHILE,
                ADD,
                    ADD,
                        Reg(2),
                        Reg(2),
                    Reg(4),
            LOAD, Reg(2), ADD,
                Reg(2),
                Reg(4),
            INPUT, Reg(2),
            WHILE, Reg(4),
                INPUT, Reg(1),
            END,
            LOAD, Reg(3), num(8),
            WHILE, Reg(2),
                LOAD, Reg(0), Reg(0),
            END,
            OUTPUT, Reg(0)
    ];
    let cases: Vec<(Vec<Number>, Vec<Number>)> = vec![
        (vec![0], vec![]),
        (vec![1], vec![1]),
        (vec![2], vec![2, 1]),
        (vec![3], vec![3, 2, 1]),
    ];
    run_cases(&program, memsize, cases);
}