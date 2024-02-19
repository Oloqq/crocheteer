#![allow(unused)]

pub mod common;
pub mod evolution;
pub mod execution;
pub mod fitness_funcs;
pub mod growing;
pub mod params;
pub mod shape;

use common::*;
use evolution::*;
use growing::*;
use params::Case;
use params::Params;

use rand::prelude::*;
use rand::SeedableRng;
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::io::Write;

use self::fitness_funcs::*;

pub fn do_genetics() {}

pub struct TinyGP {
    rand: StdRng,
    params: Params,
    cases: Vec<Case>,
    generation: i32,
    population: Vec<Program>,
    fitness: Vec<f32>,
    fitness_normalized: Vec<f64>,
    writer: RefCell<Box<dyn Write>>,
}

fn load_population(
    filepath: &str,
    params: &Params,
    cases: &Vec<Case>,
    fitness_func: FitnessFunc,
    memory_initializer: Option<&mut StdRng>,
) -> Result<(Vec<Program>, Vec<f32>), Box<dyn Error>> {
    let content = fs::read_to_string(filepath)?;
    let lines: Vec<&str> = content.trim_end().split('\n').collect();
    let mut population = Vec::with_capacity(lines.len());
    let mut fitness = Vec::with_capacity(lines.len());
    let mut memory_initializer = memory_initializer;
    // let memory_initializer = RefCell::new(memory_initializer);

    for i in 0..lines.len() {
        let program: Vec<Token> = serde_lexpr::from_str(&lines[i]).unwrap();
        population.push(program);
        fitness.push(run_and_rank(
            &population[i],
            params,
            cases,
            fitness_func,
            &mut memory_initializer,
        ));
    }

    Ok((population, fitness))
}

impl TinyGP {
    pub fn new(
        mut params: Params,
        cases: Vec<Case>,
        seed: Option<u64>,
        writer: RefCell<Box<dyn Write>>,
        fitness_func: FitnessFunc,
    ) -> TinyGP {
        let seed = seed.unwrap_or(StdRng::from_entropy().next_u64());
        let mut rand = StdRng::seed_from_u64(seed);
        params.seed = seed;
        writeln!(writer.borrow_mut(), "Creating population").unwrap();
        let (population, fitness) = random_population(&params, &cases, &mut rand, fitness_func);
        let fitness_normalized = normalize_fitness(&fitness, &population);
        TinyGP {
            rand,
            fitness,
            fitness_normalized,
            population,
            params,
            cases,
            generation: 0,
            writer: writer.into(),
        }
    }

    pub fn from_population(
        params: &Params,
        cases: &Vec<Case>,
        seed: Option<u64>,
        writer: RefCell<Box<dyn Write>>,
        fitness_func: FitnessFunc,
        filepath: &str,
    ) -> Result<TinyGP, Box<dyn Error>> {
        let seed = seed.unwrap_or(StdRng::from_entropy().next_u64());
        let mut rand = StdRng::seed_from_u64(seed);
        writeln!(writer.borrow_mut(), "Loading population").unwrap();
        let memory_init = if params.random_initial_memory {
            Some(&mut rand)
        } else {
            None
        };
        let (population, fitness) =
            load_population(filepath, &params, &cases, fitness_func, memory_init)?;
        let fitness_normalized = normalize_fitness(&fitness, &population);
        let mut params = params.clone();
        params.seed = seed;
        let case_copy: Vec<Case> = cases.clone();
        Ok(TinyGP {
            rand,
            fitness,
            fitness_normalized,
            population,
            params: params.clone(),
            cases: case_copy,
            generation: 0,
            writer: writer.into(),
        })
    }

    pub fn save_population(&self, writer: &mut Box<dyn Write>) {
        for program in self.population.iter() {
            let s = serde_lexpr::to_string(&program).unwrap();
            writeln!(writer, "{}", s).unwrap();
        }
    }

    pub fn evolve(&mut self, generations: usize, fitness_func: FitnessFunc) -> (Program, f32) {
        writeln!(
            self.writer.borrow_mut(),
            "-- TINY GP (Rust version) --\nGENERATIONS={}\n{}",
            generations,
            self.params
        )
        .unwrap();
        let mut generations = generations;
        let (mut best_fitness, mut best_id) = self.stats();
        while best_fitness < self.params.acceptable_error && generations > 0 {
            generations -= 1;
            self.evolve_generation(fitness_func);
            (best_fitness, best_id) = self.stats();
            self.writer.borrow_mut().flush().unwrap();
        }

        if best_fitness >= self.params.acceptable_error {
            writeln!(self.writer.borrow_mut(), "PROBLEM SOLVED").unwrap();
            fs::write("solution.txt", format!("{:?}", self.population[best_id])).unwrap();
        } else {
            writeln!(self.writer.borrow_mut(), "PROBLEM UNSOLVED").unwrap();
        }
        self.writer.borrow_mut().flush().unwrap();
        (self.population[best_id].clone(), best_fitness)
    }

    fn evolve_generation(&mut self, fitness_func: FitnessFunc) {
        for _ in 0..self.params.popsize {
            let child_program: Program;
            if self.rand.gen_bool(self.params.p_crossover as f64) {
                let father_id = tournament(
                    &self.fitness_normalized,
                    self.params.tournament_size,
                    &mut self.rand,
                );
                let mother_id = tournament(
                    &self.fitness_normalized,
                    self.params.tournament_size,
                    &mut self.rand,
                );
                let father = &self.population[father_id];
                let mother = &self.population[mother_id];
                let mby_overgrown = crossover(father, mother, &mut self.rand);
                if mby_overgrown.len() < self.params.max_size {
                    child_program = mby_overgrown;
                } else {
                    if self.rand.gen_bool(0.5) {
                        child_program = father.clone();
                    } else {
                        child_program = mother.clone();
                    }
                }
            } else {
                let parent_id = tournament(
                    &self.fitness_normalized,
                    self.params.tournament_size,
                    &mut self.rand,
                );
                let parent = &self.population[parent_id];
                child_program = mutation(parent, &self.params, &mut self.rand);
            };
            let child_index = negative_tournament(
                &self.fitness_normalized,
                self.params.tournament_size,
                &mut self.rand,
            );
            let mut meminit = if self.params.random_initial_memory {
                Some(&mut self.rand)
            } else {
                None
            };
            self.fitness[child_index] = run_and_rank(
                &child_program,
                &self.params,
                &self.cases,
                fitness_func,
                &mut meminit,
            );
            self.population[child_index] = child_program;
        }
        self.fitness_normalized = normalize_fitness(&self.fitness, &self.population);
        self.generation += 1;
    }

    fn stats(&mut self) -> (f32, usize) {
        let mut best = 0;
        let mut node_count = 0;
        let mut best_fitness = f32::MIN;
        let mut avg_fitness = 0.0;
        let popsize = self.population.len();

        for i in 0..popsize {
            node_count += self.population[i].len();
            avg_fitness += self.fitness[i];
            if self.fitness[i] > best_fitness {
                best = i;
                best_fitness = self.fitness[i];
            }
        }
        let avg_len = node_count / popsize;
        avg_fitness /= popsize as f32;

        writeln!(
            self.writer.borrow_mut(),
            "Generation={}
Avg Fitness={}
Best Fitness={}
Avg Size={}",
            self.generation,
            avg_fitness,
            best_fitness,
            avg_len
        )
        .unwrap();
        writeln!(self.writer.borrow_mut(), "Best Individual: ").unwrap();
        // writeln!(self.writer.borrow_mut(), "{:?}", self.population[best]);
        // pprint(&self.population[best]);
        writeln!(
            self.writer.borrow_mut(),
            "{:?}\n",
            serde_lexpr::to_string(&self.population[best]).unwrap()
        )
        .unwrap();

        (best_fitness, best)
    }
}

pub fn random_population(
    params: &Params,
    cases: &Vec<Case>,
    rand: &mut StdRng,
    fitness_func: FitnessFunc,
) -> (Vec<Program>, Vec<f32>) {
    let mut population = Vec::with_capacity(params.popsize);
    let mut fitness = Vec::with_capacity(params.popsize);

    // let memory_init = if params.random_initial_memory {
    //     Some(&rand)
    // } else {
    //     None
    // };

    for i in 0..params.popsize {
        population.push(create_random_indiv(params, rand));
        fitness.push(run_and_rank(
            &population[i],
            params,
            cases,
            fitness_func,
            &mut if params.random_initial_memory {
                Some(rand)
            } else {
                None
            },
        ));
    }

    return (population, fitness);
}

#[cfg(test)]
mod tests {}
