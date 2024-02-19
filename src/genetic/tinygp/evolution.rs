use super::execution::*;
use super::fitness_funcs::*;
use super::growing::rand_const;
use crate::params::{Case, Params};
use crate::tinygp::growing::grow_stat;

use super::common::*;
use super::growing::rand_reg;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub fn run_and_rank(
    program: &Program,
    params: &Params,
    cases: &Vec<Case>,
    fitness_func: FitnessFunc,
    memory_initializer: &mut Option<&mut StdRng>
) -> f32 {
    cases.iter().fold(0.0, |acc, (inputs, targets)| {
        let mut runtime = Runtime::new(params.memsize, &inputs, memory_initializer);
        execute(program, &mut runtime);
        let fitness = fitness_func(targets, &runtime.output, &runtime);
        // log::trace!("the fitness is: {fitness}");
        acc + fitness
    })
}

pub fn crossover(father: &Program, mother: &Program, rand: &mut StdRng) -> Program {
    log::debug!("crossover {father:?} x {mother:?}");

    if father.len() == 0 {
        return mother.clone()
    }
    if mother.len() == 0 {
        return father.clone()
    }
    let father_start = rand.gen_range(0, father.len());
    let father_kind = father[father_start];
    let father_end = get_node_end(father, father_start);

    let mother_start = match mother
        .iter()
        .enumerate()
        .filter(|(_i, v)| variant_eq(&father_kind, &v) && !matches!(father_kind, Token::Stat(Stat::IF | Stat::WHILE)))
        .choose(rand)
    {
        Some((i, _v)) => i,
        None => {
            log::warn!("parents non compatible, returning father");
            return father.clone();
        }
    };
    let mother_end = get_node_end(mother, mother_start);

    let mut offspring: Program = Vec::with_capacity(
        father_start + (mother_end - mother_start) + (father.len() - father_end),
    );
    offspring.extend_from_slice(&father[0..father_start]);
    offspring.extend_from_slice(&mother[mother_start..mother_end]);
    offspring.extend_from_slice(&father[father_end..father.len()]);
    log::trace!(" -> {offspring:?}");
    offspring
}

fn mutate_expression(source: Expr, params: &Params, rand: &mut StdRng) -> Token {
    let replacement: Token;
    // TODO implement reductive mutation (allow mutating with different argnum) (truncate the rest of the tree)
    // and also expansive mutation
    let candidate: Expr = {
        let items = &params.growing.d_expr;
        let dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
        let mut cand: Expr = items[dist2.sample(rand)].0;
        while source.argnum() != cand.argnum() {
            cand = items[dist2.sample(rand)].0;
        }
        cand
    };

    if matches!(candidate, Expr::Reg(_)) {
        replacement = rand_reg(params, rand);
    } else if matches!(candidate, Expr::Num(_)) {
        replacement = rand_const(params, rand);
    } else {
        replacement = Token::Expr(candidate);
    }
    replacement
}

pub fn mutation(parent: &Program, params: &Params, rand: &mut StdRng) -> Program {
    log::debug!("mutation of {}", serialize(parent));
    let mut child = Vec::with_capacity(parent.len());
    let mut skip_till: Option<usize> = None;
    for i in 0..parent.len() {
        if let Some(border) = skip_till {
            if i < border {
                continue;
            }
            skip_till = None
        }
        let replacement: Token;
        if rand.gen_bool(params.p_mut_per_node as f64) {
            replacement = match parent[i] {
                Token::Expr(e) => mutate_expression(e, params, rand),
                Token::Reg(_) => Token::Reg(rand.gen_range(0, params.memsize)),
                Token::Stat(_) => {
                    if rand.gen_bool(params.growing.p_insertion) {
                        child.extend(grow_stat(params.max_size as i32 - parent.len() as i32, params, rand));
                    }
                    let end = get_node_end(parent, i);
                    skip_till = Some(end);
                    child.extend(grow_stat(params.max_size as i32 - parent.len() as i32, params, rand));
                    continue;
                },
                Token::ELSE => Token::ELSE,
                Token::END => Token::END,
            }
        } else {
            replacement = parent[i];
        }
        child.push(replacement);
    }
    child
}

pub fn tournament(fitness: &Vec<f64>, tournament_size: usize, rand: &mut StdRng) -> usize {
    let mut best = rand.gen_range(0, fitness.len());
    let mut best_fitness = fitness[best];

    for _ in 0..tournament_size {
        let competitor = rand.gen_range(0, fitness.len());
        if fitness[competitor] > best_fitness {
            best_fitness = fitness[competitor];
            best = competitor;
        }
    }
    best
}

pub fn negative_tournament(fitness: &Vec<f64>, tournament_size: usize, rand: &mut StdRng) -> usize {
    let mut worst = rand.gen_range(0, fitness.len());
    let mut worst_fitness = fitness[worst];

    for _ in 0..tournament_size {
        let competitor = rand.gen_range(0, fitness.len());
        if fitness[competitor] < worst_fitness {
            worst_fitness = fitness[competitor];
            worst = competitor;
        }
    }
    worst
}

#[cfg(test)]
mod tests {
    use crate::params::GrowingParams;

    use super::*;

    #[test]
    fn test_mutate_expression() {
        let params = Params {
            memsize: 5,
            growing: GrowingParams {
                min_const: -1000,
                max_const: 1000,
                ..Default::default()
            },
            ..Default::default()
        };
        let seed = StdRng::from_entropy().next_u64();
        let mut rand = StdRng::seed_from_u64(seed);
        let source = Expr::Num(2);
        let got = mutate_expression(source, &params, &mut rand);
        match got {
            Token::Expr(Expr::Num(x)) => {
                assert!(x >= params.growing.min_const);
                assert!(x < params.growing.max_const);
            }
            #[allow(unused_comparisons)]
            Token::Reg(r) => {
                assert!(r >= 0);
                assert!(r < params.memsize);
            }
            Token::Expr(Expr::Reg(_)) => {
                panic!("mutated into the code smell Expr::R, got: {got:?}, seed = {seed}")
            }
            _ => panic!("mutation went wrong, got: {got:?}, seed = {seed}"),
        }
    }

    // #[test]
    // crossover at if and while was disabled because of this crossover producing an incorrect program with a certain rand
    // fn test_bugfix_crossover() {
    //     let params = Params {
    //         ..Default::default()
    //     };
    //     let seed = StdRng::from_entropy().next_u64();
    //     let mut rand = StdRng::seed_from_u64(seed);
    //     let prog1 = [Stat(INPUT), Reg(0), Stat(INPUT), Reg(2), Stat(IF), Reg(4), Stat(IF), Reg(2), Stat(IF), Expr(Num(-91)), Stat(LOAD), Reg(1), Expr(Num(-25)), ELSE, Stat(
    //             IF), Reg(0), ELSE, Stat(LOAD), Reg(0), Expr(Num(-58)), END, END, ELSE, Stat(IF), Expr(Num(20)), ELSE, Stat(OUTPUT), Expr(Num(59)), END, END, ELSE, Stat(OUTPUT), Reg(3),
    //             END, Stat(OUTPUT), Reg(0)];
    // }
    //  x [Stat(INPUT), Reg(0), Stat(LOAD), Reg(4), Reg(4), Stat(OUTPUT), Reg(0)]
    //     TRACE:  -> [Stat(INPUT), Reg(0), Stat(INPUT), Reg(2), Stat(IF), Reg(4), Stat(IF), Reg(2), Stat(LOAD), Reg(4), Reg(4), ELSE, Stat(OUTPUT), Reg(3), END, Stat(OUTPUT), Reg(
    //     0)]

    // #[test]
    // fn test_bugfix_mutation() {
    //     let params = Params {
    //         ..Default::default()
    //     };

    //     const INPUT: Token = Token::Stat(Stat::INPUT);
    //     const OUTPUT: Token = Token::Stat(Stat::OUTPUT);
    //     const LOAD: Token = Token::Stat(Stat::LOAD);
    //     const IF: Token = Token::Stat(Stat::IF);
    //     const WHILE: Token = Token::Stat(Stat::WHILE);
    //     const ADD: Token = Token::Expr(Expr::ADD);
    //     const ELSE: Token = Token::ELSE;
    //     const END: Token = Token::END;
    //     use Token::Reg;


    //     let seed = StdRng::from_entropy().next_u64();
    //     // let mut rand = StdRng::seed_from_u64(seed);

    //     let program = vec![INPUT, Reg(0), WHILE, Reg(2),
    //     WHILE, ADD,
    //     ADD, Reg(0), Reg(4), Reg(1), LOAD, Reg(0), Reg(0),
    //     END,
    //     INPUT, Reg(4), OUTPUT, Reg(0),
    //     END,
    //     INPUT, Reg(1), INPUT, Reg(4)];

    //     println!("node end {}", get_node_end(&program, 4));
    //     assert!(false);

    //     let result = vec![
    //         INPUT, Reg(0), WHILE, Reg(2),
    //         INPUT, Reg(3),
    //         INPUT, Reg(4),
    //         INPUT, Reg(1), INPUT, Reg(4),
    //     ];


    //     let i = vec![1, 2, 3, 4];
    //     let runtime = Runtime::new(8, &i, &mut None);
    //     let output = execute(&program, runtime);
    // }




//= ((Stat . INPUT) (Reg . 0) (Stat . WHILE) (Reg . 2)
// (Stat . INPUT) (Reg . 3)
// (Stat . INPUT) (Reg . 4)
//=  (Stat . INPUT) (Reg . 1) (Stat . INPUT) (Reg . 4))
}
