use std::{
    cell::RefCell,
    fs::File,
    io::{self, Write},
};

use crate::{
    genetic::{
        common::Program,
        params::{Case, Params},
    },
    genetic::{fitness_funcs::FitnessFunc, TinyGP},
    Args,
};

pub fn execute_benchmark(
    args: &Args,
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
        tgp = match TinyGP::from_population(&params, &cases, args.seed, writer, ff, pop_file) {
            Ok(tgp) => tgp,
            Err(_) => {
                println!("Couldn't load previous population, starting fresh");
                let writer: RefCell<Box<dyn Write>> = RefCell::new(Box::new(io::stdout()));
                TinyGP::new(params, cases, args.seed, writer, ff)
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

    let mut writer: Box<dyn Write> =
        Box::new(File::create(pop_file).expect("Could not create file"));
    tgp.save_population(&mut writer);

    tgp.get_best()
}
