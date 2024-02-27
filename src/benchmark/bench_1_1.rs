use super::util::execute_benchmark;
use crate::common::save_mesh;
use crate::genetic::common::NoInput;
use crate::genetic::fitness_funcs::*;
use crate::genetic::params::{Case, GrowingParams, Params};
use crate::genetic::shapes::Shape;
use crate::pattern::Pattern;
use crate::plushie::Plushie;
use crate::GeneticArgs;

pub fn bench_small_ball(args: &GeneticArgs) {
    let mut params = Params {
        memsize: 3,
        popsize: 100,
        max_size: 10,
        p_crossover: 0.0,
        p_mut_per_node: 0.2,
        tournament_size: 2,
        random_initial_memory: true,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.2,
            ..Default::default()
        },
        ..Default::default()
    };

    let (output, levels, max_height) = Shape::from_stl_file("src/benchmark/ball.stl").unwrap();
    params.levels = Some(levels);
    params.max_height = Some(max_height);

    let cases: Vec<Case> = vec![(NoInput {}, output)];

    let best = execute_benchmark(args, params, cases, "small_ball", shape_fitness);
    if args.save_stl {
        let pattern = Pattern::from_genom(&(6, &best));
        let mut plushie = Plushie::from_pattern(pattern);
        plushie.animate();
        save_mesh("src/benchmark/ball_generated.stl", plushie.to_mesh());
    }
}
