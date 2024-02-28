use std::{
    cell::RefCell,
    fs::File,
    io::{self, Write},
};

use crate::{
    genetic::{
        common::{Case, Program},
        fitness_funcs::FitnessFunc,
        params::Params,
        TinyGP,
    },
    GeneticArgs,
};

pub fn execute_benchmark(
    args: &GeneticArgs,
    params: Params,
    cases: Vec<Case>,
    name: &str,
    ff: FitnessFunc,
) -> Program {
    let out_file = &format!("population/out-{name}.txt");
    let pop_file = &format!("population/{name}.pop");

    let writer: RefCell<Box<dyn Write>> = if args.stdout {
        RefCell::new(Box::new(io::stdout()))
    } else {
        RefCell::new(Box::new(
            File::create(out_file).expect("Could not create file"),
        ))
    };

    let mut tgp;
    if !args.fresh {
        tgp = match TinyGP::from_population(params, cases, args.seed, writer, ff, pop_file) {
            Ok(tgp) => tgp,
            Err(e) => {
                panic!("Couldn't load previous population. Error: {e}");
            }
        }
    } else {
        tgp = TinyGP::new(params, cases, args.seed, writer, ff);
    }

    let (program, fitness) = tgp.evolve(args.generations, ff);

    println!(
        "Finished with program\n{:?}\nof fitness = {}",
        program, fitness
    );
    println!("{}", serde_lexpr::to_string(&program).unwrap());

    tgp.population.save(pop_file);
    tgp.get_best()
}
