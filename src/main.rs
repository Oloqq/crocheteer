mod args;
mod common;
mod flow;
mod meshes_sandbox;
mod pattern;
mod plushie;
mod ws_sim;
extern crate nalgebra as na;

// use crate::flow::simple_flow::SimpleFlow;
#[allow(unused)]
use crate::meshes_sandbox::*;
use crate::plushie::examples;
use crate::plushie::PlushieTrait;
use crate::plushie::{LegacyPlushie, Plushie};
use crate::{common::*, ws_sim::serve_websocket};

use args::*;
use pattern::pest_parser::{program_to_flow, Error};
use pattern::Pattern as LegacyPattern;
use std::fs;
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
            // let plushie = examples::vase_simple_flow();
            // let plushie = examples::pillar_blo();
            // let plushie = examples::pillar_simple_flow();
            // let plushie = examples::hat();
            // let plushie = examples::ergogrzib();
            let plushie = examples::fatflailer();
            let sim = PlushieSimulation::from(plushie);
            serve_websocket(sim);
        }
        Dev { num } => exec_dev_action(num),
        Genetic(genetic) => {
            let suite = &genetic.suite;
            println!("Selected suite: {suite}");
            unimplemented!();
            // run_benchmark(&suite, &genetic);
        }
        FromPattern {
            is_string,
            pattern,
            stl,
            ws,
        } => {
            let pattern = if is_string {
                unimplemented!()
                // let tokens = Program::deserialize(pattern.to_str().unwrap())
                //     .unwrap()
                //     .tokens;
                // Pattern::from_genom(&(6, &tokens))
            } else {
                // LegacyPattern::from_file(pattern).unwrap()
                let content = fs::read_to_string(&pattern).unwrap();
                match program_to_flow(&content) {
                    Ok(val) => val,
                    Err(Error::Lexer(e)) => {
                        println!("{e}");
                        return;
                    }
                    Err(e) => panic!("{e}"),
                }
            };
            // let flow = SimpleFlow::from_legacy_pattern(pattern);
            let flow = pattern;
            let plushie = Plushie::from_flow(flow).unwrap();

            if stl.is_some() && ws || stl.is_none() && !ws {
                unimplemented!("use either --stl or --ws");
            }

            if let Some(_stl_path) = stl {
                unimplemented!()
                // plushie.animate();
                // save_mesh(stl_path.to_str().unwrap(), plushie.to_mesh());
            } else if ws {
                let sim = PlushieSimulation::from(plushie);
                serve_websocket(sim);
            }
        }
        FromProtoPattern { protopat: _ } => todo!(),
    }
}

fn exec_dev_action(num: usize) {
    fn generate(name: &str, func: fn() -> (LegacyPattern, LegacyPlushie)) {
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
