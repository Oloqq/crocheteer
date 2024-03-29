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
use serde_derive::Serialize;
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
    pub debug_info: bool,
}

impl TinyGP {
    fn start_construction(
        writer: &RefCell<Box<dyn Write>>,
        seed: Option<u64>,
        msg: &str,
    ) -> StdRng {
        let seed = seed.unwrap_or(StdRng::from_entropy().next_u64());
        writeln!(writer.borrow_mut(), "---\n{}\n---\nseed: {}", msg, seed).unwrap();
        StdRng::seed_from_u64(seed)
    }

    fn init(
        rand: StdRng,
        population: Population,
        params: Params,
        cases: Vec<Case>,
        writer: RefCell<Box<dyn Write>>,
    ) -> Self {
        let obj = TinyGP {
            rand,
            population,
            params,
            cases,
            generation: 0,
            writer: writer.into(),
            debug_info: false,
        };
        obj.write(serde_yaml::to_string(&obj.params).unwrap().as_str());
        obj
    }

    pub fn new(
        params: Params,
        cases: Vec<Case>,
        seed: Option<u64>,
        writer: RefCell<Box<dyn Write>>,
        fitness_func: FitnessFunc,
    ) -> TinyGP {
        let mut rand = Self::start_construction(&writer, seed, "Creating population");
        let population = Population::make_random(&params, &cases, &mut rand, fitness_func);

        Self::init(rand, population, params, cases, writer)
    }

    fn write(&self, msg: &str) {
        writeln!(self.writer.borrow_mut(), "---\n{msg}").unwrap();
    }

    pub fn from_population(
        params: Params,
        cases: Vec<Case>,
        seed: Option<u64>,
        writer: RefCell<Box<dyn Write>>,
        fitness_func: FitnessFunc,
        filepath: &str,
    ) -> Result<TinyGP, Box<dyn Error>> {
        let mut rand = Self::start_construction(&writer, seed, "Loading population");
        let population = Population::load(filepath, &params, &cases, fitness_func, &mut rand)?;

        Ok(Self::init(rand, population, params, cases, writer))
    }

    pub fn evolve(
        &mut self,
        mut generations: usize,
        fitness_func: FitnessFunc,
    ) -> Option<(Program, f32)> {
        self.write(format!("Starting evolution for {generations} generations").as_str());

        let (mut best_id, mut best_fitness) = self.population.get_best_id();
        while best_fitness < self.params.acceptable_error && generations > 0 {
            self.report_progress();
            generations -= 1;
            self.evolve_generation(fitness_func);
            (best_id, best_fitness) = self.population.get_best_id();
            self.writer.borrow_mut().flush().unwrap();
        }

        if best_fitness >= self.params.acceptable_error {
            self.write("Problem solved");
            fs::write(
                "solution.txt",
                format!("{}", self.population.programs[best_id].serialize()),
            )
            .unwrap();
        } else {
            self.write("Problem unsolved");
        }
        self.writer.borrow_mut().flush().unwrap();
        Some(self.population.get_best())
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

            self.population.emplace(
                child_index,
                child_program,
                &self.params,
                &self.cases,
                fitness_func,
                &mut self.rand,
            );
        }
        self.generation += 1;
    }

    pub fn get_best(&mut self) -> Program {
        self.population.get_best().0
    }

    fn report_progress(&self) {
        #[derive(Serialize)]
        struct Raport {
            generation: i32,
            avg_fitness: f32,
            best_fitness: f32,
            best_program: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            programs: Option<Vec<(f32, String)>>,
        }

        let (best_program, best_fitness) = self.population.get_best();
        let programs = if self.debug_info {
            let progs: Vec<(f32, String)> = self
                .population
                .programs
                .iter()
                .zip(&self.population.fitness)
                .map(|(prog, fit)| (*fit, prog.serialize()))
                .collect();
            Some(progs)
        } else {
            None
        };

        let r = Raport {
            generation: self.generation,
            avg_fitness: self.population.average_fitness(),
            best_program: best_program.serialize(),
            best_fitness,
            programs,
        };

        let s = serde_yaml::to_string(&r).unwrap();
        writeln!(self.writer.borrow_mut(), "---\n{}", s).unwrap();
    }
}
