use crate::benchmark::run_benchmark;
use crate::genetic::common::Program;
#[allow(unused)]
use crate::meshes_sandbox::*;
use crate::{common::*, ws_sim::serve_websocket};

extern crate nalgebra as na;

mod args;
mod benchmark;
mod common;
mod flow;
mod genetic;
mod meshes_sandbox;
mod pattern;
mod plushie;
mod plushie2;
mod ws_sim;

use args::*;
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
    use Command::*;
    match args.cmd {
        WebSocket {} => {
            let (_, plushie) = examples::vase();
            let sim = PlushieSimulation::from(plushie);
            serve_websocket(sim);
        }
        Dev { num } => exec_dev_action(num),
        Genetic(genetic) => {
            let suite = &genetic.suite;
            println!("Selected suite: {suite}");
            run_benchmark(&suite, &genetic);
        }
        FromPattern {
            is_string,
            pattern,
            stl,
            ws,
        } => {
            let pattern = if is_string {
                let tokens = Program::deserialize(pattern.to_str().unwrap())
                    .unwrap()
                    .tokens;
                Pattern::from_genom(&(6, &tokens))
            } else {
                Pattern::from_file(pattern).unwrap()
            };
            let mut plushie = Plushie::from_pattern(&pattern);

            if stl.is_some() && ws || stl.is_none() && !ws {
                unimplemented!("use either --stl or --ws");
            }

            if let Some(stl_path) = stl {
                plushie.animate();
                save_mesh(stl_path.to_str().unwrap(), plushie.to_mesh());
            } else if ws {
                let sim = PlushieSimulation::from(plushie);
                serve_websocket(sim);
            }
        }
        FromProtoPattern { protopat: _ } => todo!(),
    }
}

fn exec_dev_action(num: usize) {
    fn generate(name: &str, func: fn() -> (Pattern, Plushie)) {
        let (_pat, mut plushie) = func();
        // println!(
        //     "{:?}",
        //     plushie.points.iter().map(|a| a.y).collect::<Vec<_>>()
        // );
        plushie.animate();
        // println!(
        //     "{:?}",
        //     plushie.points.iter().map(|a| a.y).collect::<Vec<_>>()
        // );
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
