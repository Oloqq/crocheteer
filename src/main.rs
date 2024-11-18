mod acl;
mod args;
mod common;
mod plushie;
mod ws_sim;

extern crate nalgebra as na;

use self::args::*;
use self::ws_sim::plushie_sim::PlushieSimulation;
use crate::acl::pest_parser::Pattern;
use crate::plushie::examples;
use crate::plushie::Params;
use crate::plushie::{Plushie, Pointcloud};
use crate::ws_sim::serve_websocket;
use plushie::params;
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
            let presetable = args.apply_preset();

            let mut plushie = match examples::get_example(&presetable.plushie) {
                Some(x) => x,
                None => {
                    log::error!("Plushie {:?} does not exist", args.plushie);
                    return;
                }
            };
            plushie.params = match params::handpicked::get(&presetable.params) {
                Some(x) => x,
                None => {
                    log::error!("Params {:?} does not exist", args.params);
                    return;
                }
            };
            let sim = match presetable.secondary {
                // why bother with PathBuf when String does the job
                // forums say that its due to Rust strings handling of UTF-8
                // so I guess the filesystem will go apeshit if I e.g. pass emoji as argument?
                // TODO investigate
                Some(path) => {
                    let secondary_plushie =
                        Pointcloud::from_points_file(path.to_str().expect("converted to str"));
                    PlushieSimulation::with_secondary(plushie, secondary_plushie)
                }
                None => PlushieSimulation::from(plushie),
            };

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
                    // viewing vertexes in an STL
                    let plushie = Pointcloud::from_stl("models/grzib40.stl");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                4 => {
                    // viewing a pregenerated pointcloud
                    let plushie =
                        Pointcloud::from_points_file("model_preprocessing/models/pillar.json");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                5 => {
                    // visually compare a pointcloud to an example
                    let mut primary = examples::pillar();
                    primary.params = params::handpicked::grzib();
                    let secondary =
                        Pointcloud::from_points_file("model_preprocessing/models/pillar.json");
                    let sim = PlushieSimulation::with_secondary(primary, secondary);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                11 => {
                    let mut plushie = examples::ergogrzob();
                    plushie.params = params::handpicked::grzob();
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                _ => {}
            }
            println!(":)");
            println!(":)");
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
