pub mod common;
pub mod evolution;
pub mod execution;
pub mod fitness_funcs;
pub mod growing;
pub mod params;
pub mod problem;

use common::*;
use evolution::*;
use params::Params;

use rand::prelude::*;
use rand::SeedableRng;
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::io::Write;

use self::fitness_funcs::*;

pub struct TinyGP {
    rand: StdRng,
    params: Params,
    cases: Vec<Case>,
    generation: i32,
    pub population: Population,
    writer: RefCell<Box<dyn Write>>,
}

impl TinyGP {
    pub fn new(
        params: Params,
        cases: Vec<Case>,
        seed: Option<u64>,
        writer: RefCell<Box<dyn Write>>,
        fitness_func: FitnessFunc,
    ) -> TinyGP {
        let seed = seed.unwrap_or(StdRng::from_entropy().next_u64());
        let mut rand = StdRng::seed_from_u64(seed);
        writeln!(writer.borrow_mut(), "Creating population").unwrap();

        let population = Population::make_random(&params, &cases, &mut rand, fitness_func);

        TinyGP {
            rand,
            population,
            params,
            cases,
            generation: 0,
            writer: writer.into(),
        }
    }

    pub fn from_population(
        params: Params,
        cases: Vec<Case>,
        seed: Option<u64>,
        writer: RefCell<Box<dyn Write>>,
        fitness_func: FitnessFunc,
        filepath: &str,
    ) -> Result<TinyGP, Box<dyn Error>> {
        let seed = seed.unwrap_or(StdRng::from_entropy().next_u64());
        let mut rand = StdRng::seed_from_u64(seed);
        writeln!(writer.borrow_mut(), "Loading population").unwrap();

        let population = Population::load(filepath, &params, &cases, fitness_func, &mut rand)?;
        Ok(TinyGP {
            rand,
            population,
            params,
            cases,
            generation: 0,
            writer: writer.into(),
        })
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
            fs::write(
                "solution.txt",
                format!("{}", self.population.programs[best_id].serialize()),
            )
            .unwrap();
        } else {
            writeln!(self.writer.borrow_mut(), "PROBLEM UNSOLVED").unwrap();
        }
        self.writer.borrow_mut().flush().unwrap();
        (self.population.programs[best_id].clone(), best_fitness)
    }

    fn evolve_generation(&mut self, fitness_func: FitnessFunc) {
        for _ in 0..self.params.popsize {
            let child_program: Program;
            if self.rand.gen_bool(self.params.p_crossover as f64) {
                let father_id = tournament(
                    &self.population.fitness,
                    self.params.tournament_size,
                    &mut self.rand,
                );
                let mother_id = tournament(
                    &self.population.fitness,
                    self.params.tournament_size,
                    &mut self.rand,
                );
                let father = &self.population.programs[father_id];
                let mother = &self.population.programs[mother_id];
                let mby_overgrown = crossover(father, mother, &mut self.rand);
                if mby_overgrown.tokens.len() < self.params.max_size {
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
                    &self.population.fitness,
                    self.params.tournament_size,
                    &mut self.rand,
                );
                let parent = &self.population.programs[parent_id];
                child_program = mutation(parent, &self.params, &mut self.rand);
            };
            let child_index = negative_tournament(
                &self.population.fitness,
                self.params.tournament_size,
                &mut self.rand,
            );
            self.population.fitness[child_index] = run_and_rank(
                &child_program,
                &self.params,
                &self.cases,
                fitness_func,
                &mut self.rand,
            );
            self.population.programs[child_index] = child_program;
        }
        self.generation += 1;
    }

    pub fn get_best(&mut self) -> Program {
        let (_, besti) = self.stats();
        self.population.programs[besti].clone()
    }

    fn stats(&mut self) -> (f32, usize) {
        let mut best = 0;
        let mut node_count = 0;
        let mut best_fitness = f32::MIN;
        let mut avg_fitness = 0.0;
        let popsize = self.population.programs.len();

        for i in 0..popsize {
            node_count += self.population.programs[i].tokens.len();
            avg_fitness += self.population.fitness[i];
            if self.population.fitness[i] > best_fitness {
                best = i;
                best_fitness = self.population.fitness[i];
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
            serde_lexpr::to_string(&self.population.programs[best]).unwrap()
        )
        .unwrap();

        (best_fitness, best)
    }
}
