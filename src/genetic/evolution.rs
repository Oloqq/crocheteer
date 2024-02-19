use super::execution::*;
use super::fitness_funcs::*;
use super::params::{Case, Params};

use super::common::*;
// use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub fn run_and_rank(
    program: &Program,
    params: &Params,
    cases: &Vec<Case>,
    fitness_func: FitnessFunc,
    memory_initializer: &mut Option<&mut StdRng>,
) -> f32 {
    cases.iter().fold(0.0, |acc, (inputs, targets)| {
        let mut runtime = Runtime::new(params);
        let output = runtime.execute(program);
        let fitness = fitness_func(targets, &output, &runtime);
        // log::trace!("the fitness is: {fitness}");
        acc + fitness
    })
}

pub fn crossover(father: &Program, mother: &Program, rand: &mut StdRng) -> Program {
    log::debug!("crossover {father:?} x {mother:?}");
    todo!()

    // if father.len() == 0 {
    //     return mother.clone();
    // }
    // if mother.len() == 0 {
    //     return father.clone();
    // }
    // let father_start = rand.gen_range(0, father.len());
    // let father_kind = father[father_start];
    // let father_end = get_node_end(father, father_start);

    // let mother_start = match mother
    //     .iter()
    //     .enumerate()
    //     .filter(|(_i, v)| {
    //         variant_eq(&father_kind, &v)
    //             && !matches!(father_kind, Token::Stat(Stat::IF | Stat::WHILE))
    //     })
    //     .choose(rand)
    // {
    //     Some((i, _v)) => i,
    //     None => {
    //         log::warn!("parents non compatible, returning father");
    //         return father.clone();
    //     }
    // };
    // let mother_end = get_node_end(mother, mother_start);

    // let mut offspring: Program = Vec::with_capacity(
    //     father_start + (mother_end - mother_start) + (father.len() - father_end),
    // );
    // offspring.extend_from_slice(&father[0..father_start]);
    // offspring.extend_from_slice(&mother[mother_start..mother_end]);
    // offspring.extend_from_slice(&father[father_end..father.len()]);
    // log::trace!(" -> {offspring:?}");
    // offspring
}

// fn mutate_expression(source: Expr, params: &Params, rand: &mut StdRng) -> Token {
//     let replacement: Token;
//     // TODO implement reductive mutation (allow mutating with different argnum) (truncate the rest of the tree)
//     // and also expansive mutation
//     let candidate: Expr = {
//         let items = &params.growing.d_expr;
//         let dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
//         let mut cand: Expr = items[dist2.sample(rand)].0;
//         while source.argnum() != cand.argnum() {
//             cand = items[dist2.sample(rand)].0;
//         }
//         cand
//     };

//     if matches!(candidate, Expr::Reg(_)) {
//         replacement = rand_reg(params, rand);
//     } else if matches!(candidate, Expr::Num(_)) {
//         replacement = rand_const(params, rand);
//     } else {
//         replacement = Token::Expr(candidate);
//     }
//     replacement
// }

pub fn mutation(parent: &Program, params: &Params, rand: &mut StdRng) -> Program {
    log::debug!("mutation of {}", serialize(parent));
    todo!()
    // let mut child = Vec::with_capacity(parent.len());
    // let mut skip_till: Option<usize> = None;
    // for i in 0..parent.len() {
    //     if let Some(border) = skip_till {
    //         if i < border {
    //             continue;
    //         }
    //         skip_till = None
    //     }
    //     let replacement: Token;
    //     if rand.gen_bool(params.p_mut_per_node as f64) {
    //         replacement = match parent[i] {
    //             Token::Expr(e) => mutate_expression(e, params, rand),
    //             Token::Reg(_) => Token::Reg(rand.gen_range(0, params.memsize)),
    //             Token::Stat(_) => {
    //                 if rand.gen_bool(params.growing.p_insertion) {
    //                     child.extend(grow_stat(
    //                         params.max_size as i32 - parent.len() as i32,
    //                         params,
    //                         rand,
    //                     ));
    //                 }
    //                 let end = get_node_end(parent, i);
    //                 skip_till = Some(end);
    //                 child.extend(grow_stat(
    //                     params.max_size as i32 - parent.len() as i32,
    //                     params,
    //                     rand,
    //                 ));
    //                 continue;
    //             }
    //             Token::ELSE => Token::ELSE,
    //             Token::END => Token::END,
    //         }
    //     } else {
    //         replacement = parent[i];
    //     }
    //     child.push(replacement);
    // }
    // child
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
