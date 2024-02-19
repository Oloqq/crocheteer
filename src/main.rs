use crate::benchmark::run_benchmark;
#[allow(unused)]
use crate::meshes_sandbox::*;
use crate::{common::*, ws_sim::serve_websocket};

extern crate nalgebra as na;

mod args;
mod benchmark;
mod common;
mod genetic;
mod meshes_sandbox;
mod pattern;
mod plushie;
mod ws_sim;

use args::*;
use genetic::do_genetics;
use pattern::Pattern;
use plushie::examples;
use plushie::Plushie;
use std::io::Write;
use ws_sim::plushie_sim::PlushieSimulation;

fn main() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let args = Args::from_args();
    if let Some(num) = args.dev {
        exec_dev_action(num);
        return;
    }

    if args.genetic {
        do_genetics();
        return;
    }

    if let Some(suite) = &args.suite {
        println!("Selected suite: {suite}");
        run_benchmark(&suite, &args);
    }

    if let Some(pattern_path) = args.pattern {
        let pattern = Pattern::from_file(pattern_path);
        let mut plushie = Plushie::from_pattern(pattern);

        if let Some(stl_path) = args.stl {
            plushie.animate();
            save_mesh(stl_path.to_str().unwrap(), plushie.to_mesh());
        } else if args.ws {
            let sim = PlushieSimulation::from(plushie);
            serve_websocket(sim);
        }
    } else if args.ws {
        let plushie = examples::bigball();
        let sim = PlushieSimulation::from(plushie);
        serve_websocket(sim);
    }
}

fn exec_dev_action(num: usize) {
    fn generate(name: &str, func: fn() -> Plushie) {
        let mut plushie = func();
        plushie.animate();
        save_mesh(name, plushie.to_mesh());
    }

    println!("dev action {num}");
    match num {
        2 => generate("generated/pillar.stl", examples::pillar),
        3 => generate("generated/ball.stl", examples::ball),
        4 => generate("generated/bigball.stl", examples::bigball),
        _ => println!("no such action"),
    }
}
