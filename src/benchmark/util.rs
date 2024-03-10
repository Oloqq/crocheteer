use std::{
    cell::RefCell,
    fs::File,
    io::{self, Write},
};

use crate::{
    common::save_mesh,
    genetic::{common::Case, fitness_funcs::FitnessFunc, params::Params, problem::Shape, TinyGP},
    pattern::Pattern,
    plushie::Plushie,
    GeneticArgs,
};

pub fn execute_benchmark(
    args: &GeneticArgs,
    params: Params,
    cases: Vec<Case>,
    name: &str,
    ff: FitnessFunc,
) {
    let out_file = &format!("population/out-{name}.yaml");
    let pop_file = &format!("population/{name}.pop");

    let writer: RefCell<Box<dyn Write>> = if args.stdout {
        RefCell::new(Box::new(io::stdout()))
    } else {
        RefCell::new(Box::new(
            File::create(out_file).expect("Could not create file"),
        ))
    };

    if cases.len() == 1 {
        let expected: &Shape = &cases[0].1;
        writeln!(
            writer.borrow_mut(),
            "Comparing to: {}",
            expected.serialize()
        )
        .unwrap();
    }

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

    tgp.debug_info = args.debug;

    let result = tgp.evolve(args.generations, ff);
    tgp.population.save(pop_file);
    let (program, fitness) = result.expect("Benchmark execution failed");

    println!(
        "Finished with program\n{:?}\nof fitness = {}",
        program, fitness
    );
    println!("{}", serde_lexpr::to_string(&program).unwrap());

    let best = tgp.get_best();

    if args.save_stl {
        let pattern = Pattern::from_genom(&(6, &best.tokens));
        let mut plushie = Plushie::from_pattern(&pattern);
        plushie.animate();
        save_mesh(
            format!("src/benchmark/{name}_generated.stl").as_str(),
            plushie.to_mesh(),
        );
    }
}
