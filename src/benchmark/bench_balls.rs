use super::util::execute_benchmark;
use crate::common::save_mesh;
use crate::genetic::common::Case;
use crate::genetic::fitness_funcs::*;
use crate::genetic::params::Params;
use crate::genetic::problem::NoInput;
use crate::genetic::problem::Shape;
use crate::pattern::Pattern;
use crate::plushie::Plushie;
use crate::GeneticArgs;

pub fn bench_small_ball(args: &GeneticArgs) {
    let mut params = Params {
        popsize: 100,
        max_size: 40,
        p_crossover: 0.0,
        p_mutation_per_node: 0.2,
        tournament_size: 2,
        ..Default::default()
    };

    let (output, levels, max_height) =
        Shape::from_stl_file("src/benchmark/small_ball.stl").unwrap();
    params.levels = Some(levels);
    params.max_height = Some(max_height);

    let cases: Vec<Case> = vec![(NoInput {}, output)];

    let best = execute_benchmark(args, params, cases, "small_ball", shape_fitness);
    if args.save_stl {
        let pattern = Pattern::from_genom(&(6, &best.tokens));
        let mut plushie = Plushie::from_pattern(&pattern);
        plushie.animate();
        save_mesh("src/benchmark/small_ball_generated.stl", plushie.to_mesh());
    }
}

pub fn bench_big_ball(args: &GeneticArgs) {
    let mut params = Params {
        popsize: 100,
        max_size: 40,
        p_crossover: 0.0,
        p_mutation_per_node: 0.2,
        tournament_size: 2,
        ..Default::default()
    };

    let (output, levels, max_height) = Shape::from_stl_file("src/benchmark/big_ball.stl").unwrap();
    params.levels = Some(levels);
    params.max_height = Some(max_height);

    let cases: Vec<Case> = vec![(NoInput {}, output)];

    let best = execute_benchmark(args, params, cases, "big_ball", shape_fitness);
    if args.save_stl {
        let pattern = Pattern::from_genom(&(6, &best.tokens));
        let mut plushie = Plushie::from_pattern(&pattern);
        plushie.animate();
        save_mesh("src/benchmark/big_ball_generated.stl", plushie.to_mesh());
    }
}
