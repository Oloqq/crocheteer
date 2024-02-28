pub use super::problem::{Input, Output, Token};

use super::evolution::*;
use super::{fitness_funcs::FitnessFunc, params::Params};
use rand::rngs::StdRng;
use serde_derive::{Deserialize, Serialize};
use std::f32::NAN;
use std::fs::File;
use std::io::Write;
use std::{error::Error, fs};

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
        serde_lexpr::to_string(&self.tokens).unwrap()
    }

    pub fn deserialize(src: &str) -> Result<Self, String> {
        match serde_lexpr::from_str(src) {
            Ok(prog) => Ok(Self { tokens: prog }),
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
    best_fitness: f32,
    best_index: usize,
}

impl Population {
    fn init(
        programs: Vec<Program>,
        params: &Params,
        cases: &Vec<Case>,
        rand: &mut StdRng,
        fitness_func: FitnessFunc,
    ) -> Self {
        let mut fitness = Vec::with_capacity(programs.len());
        let mut best_fitness = f32::MIN;
        let mut best_index = 0;

        assert!(params.popsize == programs.len());
        for i in 0..programs.len() {
            let fit = run_and_rank(&programs[i], params, cases, fitness_func, rand);
            if fit > best_fitness {
                best_fitness = fit;
                best_index = i;
            }
            fitness.push(fit);
        }

        Self {
            programs,
            fitness,
            best_fitness,
            best_index,
        }
    }

    pub fn make_random(
        params: &Params,
        cases: &Vec<Case>,
        rand: &mut StdRng,
        fitness_func: FitnessFunc,
    ) -> Self {
        use super::growing::create_random_indiv;

        let programs = (0..params.popsize)
            .map(|_| create_random_indiv(params, rand))
            .collect();

        Self::init(programs, params, cases, rand, fitness_func)
    }

    pub fn save(&self, filepath: &str) {
        let mut file = File::create(filepath).unwrap();
        for program in &self.programs {
            writeln!(file, "{}", program.serialize()).unwrap()
        }
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

        assert!(programs.len() == params.popsize);
        Ok(Self::init(programs, params, cases, rand, fitness_func))
    }

    pub fn emplace(
        &mut self,
        index: usize,
        program: Program,
        params: &Params,
        cases: &Vec<Case>,
        fitness_func: FitnessFunc,
        rand: &mut StdRng,
    ) {
        assert!(index < self.programs.len());
        let fit = run_and_rank(&program, &params, &cases, fitness_func, rand);
        if fit > self.best_fitness {
            self.best_fitness = fit;
            self.best_index = index;
        }
        self.fitness[index] = fit;
        self.programs[index] = program;
    }

    pub fn get_best(&self) -> (Program, f32) {
        assert!(self.best_fitness == self.fitness[self.best_index]);
        (self.programs[self.best_index].clone(), self.best_fitness)
    }

    pub fn get_best_id(&self) -> (usize, f32) {
        assert!(self.best_fitness == self.fitness[self.best_index]);
        (self.best_index, self.best_fitness)
    }

    pub fn average_fitness(&self) -> f32 {
        let sum: f32 = self.fitness.iter().fold(0.0, |acc, f| {
            // assert!(!f.is_nan(), "NaN fitness");
            acc + f
        });
        sum / self.fitness.len() as f32
    }
}
