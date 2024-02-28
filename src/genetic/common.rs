use std::{error::Error, fs};

use rand::rngs::StdRng;
use serde_derive::{Deserialize, Serialize};

pub use super::problem::{Input, Output, Token};
use super::{fitness_funcs::FitnessFunc, params::Params};

// pub trait AnyProgram {
//     fn serialize(&self) -> String;
//     fn deserialize(src: &str) -> Self;
//     fn judge(fitness_func: FitnessFunc) -> f32;
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub tokens: Vec<Token>,
}

impl Program {
    pub fn serialize(&self) -> String {
        serde_lexpr::to_string(&self).unwrap()
    }

    pub fn deserialize(src: &str) -> Result<Self, String> {
        match serde_lexpr::from_str(src) {
            Ok(prog) => Ok(prog),
            Err(_) => Err(format!("Couldn't load program: {src}")),
        }
    }

    // fn judge(fitness_func: FitnessFunc) -> f32 {
    //     todo!()
    // }
}

pub type Case = (Input, Output);

// pub trait AnyPopulation {
//     fn make_random() -> Self;
// }

pub struct Population {
    pub programs: Vec<Program>,
    pub fitness: Vec<f32>,
}

impl Population {
    pub fn make_random(
        params: &Params,
        cases: &Vec<Case>,
        rand: &mut StdRng,
        fitness_func: FitnessFunc,
    ) -> Self {
        let (programs, fitness) = random_population(&params, &cases, rand, fitness_func);
        Population { programs, fitness }
    }

    pub fn load(
        filepath: &str,
        params: &Params,
        cases: &Vec<Case>,
        fitness_func: FitnessFunc,
        rand: &mut StdRng,
    ) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(filepath)?;
        let lines: Vec<&str> = content.trim_end().split('\n').collect();

        let programs: Vec<Program> = {
            let mut programs = Vec::with_capacity(lines.len());
            for line in lines {
                programs.push(Program::deserialize(line)?);
            }
            programs
        };

        let fitness = programs
            .iter()
            .map(|program| run_and_rank(program, params, cases, fitness_func, rand))
            .collect();

        assert!(programs.len() == params.popsize);

        Ok(Self { programs, fitness })
    }
}

use super::evolution::*;
use super::growing::*;

fn random_population(
    params: &Params,
    cases: &Vec<Case>,
    rand: &mut StdRng,
    fitness_func: FitnessFunc,
) -> (Vec<Program>, Vec<f32>) {
    let mut population = Vec::with_capacity(params.popsize);
    let mut fitness = Vec::with_capacity(params.popsize);

    for i in 0..params.popsize {
        population.push(create_random_indiv(params, rand));
        fitness.push(run_and_rank(
            &population[i],
            params,
            cases,
            fitness_func,
            rand,
        ));
    }

    return (population, fitness);
}
