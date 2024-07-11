mod args;
mod common;
mod comparison;
mod flow;
mod plushie;
mod ws_sim;
extern crate nalgebra as na;

use self::args::*;
use self::ws_sim::plushie_sim::PlushieSimulation;
use crate::flow::pest_parser::Pattern;
use crate::plushie::examples;
use crate::plushie::Params;
use crate::plushie::{Plushie, Pointcloud};
use crate::ws_sim::serve_websocket;
use std::fs;
use std::io::Write;

fn main() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let args = Args::from_args();
    use Command::*;
    match args.cmd {
        WebSocket(args) => {
            let plushie = examples::ergogrzib();
            let sim = PlushieSimulation::from(plushie);
            serve_websocket(sim, format!("127.0.0.1:{}", args.port).as_str());
        }
        Dev { num } => {
            match num {
                1 => {
                    let d = Params::default();
                    let s = serde_json::to_string_pretty(&d).unwrap();
                    println!("{s}");
                }
                2 => {
                    let plushie = examples::ergogrzib();
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                3 => {
                    let plushie = Pointcloud::from_stl("models/grzib40.stl");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                4 => {
                    let plushie = Pointcloud::from_points_file("models/grzib5cloud1000.json");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                5 => {
                    let primary = examples::ergogrzib();
                    let secondary = Pointcloud::from_points_file("models/grzib10cloud1000.json");
                    let sim = PlushieSimulation::with_secondary(primary, secondary);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                6 => {
                    let plushie =
                        Pointcloud::from_points_file("model_preprocessing/pointcloud.json");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                _ => {}
            }
            println!(":)");
            println!(":)");
        }
        Genetic(genetic) => {
            let suite = &genetic.suite;
            println!("Selected suite: {suite}");
            unimplemented!();
            // run_benchmark(&suite, &genetic);
        }
        FromPattern { pattern, stl, ws } => {
            let pattern = {
                let content = fs::read_to_string(&pattern).unwrap();
                match Pattern::parse(&content) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("{e}");
                        return;
                    }
                }
            };
            let mut params: Params = Default::default();
            params.update(&pattern.meta);
            let plushie = Plushie::from_flow(pattern, params).unwrap();

            if stl.is_some() && ws || stl.is_none() && !ws {
                println!("use either --stl or --ws");
                return;
            }

            if let Some(_stl_path) = stl {
                unimplemented!()
                // plushie.animate();
                // save_mesh(stl_path.to_str().unwrap(), plushie.to_mesh());
            } else if ws {
                let sim = PlushieSimulation::from(plushie);
                serve_websocket(sim, "127.0.0.1:8080");
            }
        }
    }
}
