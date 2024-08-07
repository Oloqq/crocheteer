// #![allow(unused)]
use std::cell::RefCell;
use std::io::Write;
use std::ops::DerefMut;

use super::execution::*;
use super::fitness_funcs::*;
use super::params::Params;

use super::common::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use tokio_tungstenite::tungstenite::http::uri::PathAndQuery;

pub fn run_and_rank(
    program: &Program,
    params: &Params,
    cases: &Vec<Case>,
    fitness_func: FitnessFunc,
    rand: &mut StdRng,
    writer: RefCell<Box<dyn Write>>,
) -> f32 {
    cases.iter().fold(0.0, |acc, (inputs, targets)| {
        let mut runtime = Runtime::new(params);
        let output = runtime.execute(program);
        let fitness = fitness_func(targets, &output, &runtime);
        if fitness.is_nan() {
            writeln!(writer.borrow_mut(), "Got fitness NaN with:").unwrap();
            runtime.log(&writer);
        }
        acc + fitness
    })
}

pub fn crossover(father: &Program, mother: &Program, rand: &mut StdRng) -> Program {
    unimplemented!(
        "Crossover is not implemented for this project. Set probability of crossover to 0.0"
    );
}

pub fn mutation(parent: &Program, params: &Params, rand: &mut StdRng) -> Program {
    log::debug!("mutation of {}", parent.serialize());

    #[derive(Clone, Copy)]
    enum Operation {
        Mutate,
        Duplicate,
        Remove,
    }

    // TODO params
    let weights = vec![
        (Operation::Mutate, 2.0),
        (Operation::Duplicate, 1.0),
        (Operation::Remove, 1.0),
    ];

    let mut child = Vec::with_capacity(parent.tokens.len());
    for i in 0..parent.tokens.len() {
        let operation: Operation = {
            let items = &weights;
            let distribution = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
            items[distribution.sample(rand)].0
        };

        match operation {
            Operation::Mutate => {
                let replacement: Token = rand.gen(); // TODO distribution
                child.push(replacement);
            }
            Operation::Duplicate => {
                child.push(parent.tokens[i]);
                child.push(parent.tokens[i]);
            }
            Operation::Remove => continue,
        }
    }
    if child.len() > params.max_size {
        child.split_off(params.max_size);
    }
    Program { tokens: child }
}

pub fn tournament(fitness: &Vec<f32>, tournament_size: usize, rand: &mut StdRng) -> usize {
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

pub fn negative_tournament(fitness: &Vec<f32>, tournament_size: usize, rand: &mut StdRng) -> usize {
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
