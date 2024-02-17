#[allow(unused)]
use crate::meshes_sandbox::*;
use crate::pattern::Stitch;
use crate::{common::*, ws_sim::serve_websocket};

extern crate nalgebra as na;

mod args;
mod common;
mod meshes_sandbox;
mod pattern;
mod plushie;
mod ws_sim;

use args::*;
use pattern::Pattern;
use plushie::examples;
use plushie::Plushie;
use ws_sim::plushie_sim::PlushieSimulation;

fn main() {
    let args = Args::from_args();
    if let Some(num) = args.dev {
        exec_dev_action(num);
        return;
    }

    if let Some(pattern_path) = args.pattern {
        let pattern = Pattern::from_file(pattern_path);
        let mut plushie = Plushie::from_pattern(pattern);

        if let Some(stl_path) = args.stl {
            plushie.animate();
            save_mesh(stl_path.to_str().unwrap(), plushie.to_mesh());
        } else if args.ws {
            unimplemented!();
            // let sim = PlushieSimulation::from(plushie);
            // serve_websocket(sim);
        }
    } else if args.ws {
        let plushie = examples::bigball();
        let sim = PlushieSimulation::from(plushie);
        serve_websocket(sim);
    }
}

fn exec_dev_action(num: usize) {
    println!("dev action {num}");
    match num {
        1 => save_and_stuff_diamnond(),
        2 => make_pillar(),
        3 => make_ball(),
        4 => make_big_ball(),
        _ => println!("no such action"),
    }
}

fn make_pillar() {
    let mut plushie = examples::pillar();
    plushie.animate();
    save_mesh("generated/pillar.stl", plushie.to_mesh());
}

fn make_ball() {
    let mut plushie = examples::ball();
    plushie.animate();
    save_mesh("generated/ball.stl", plushie.to_mesh());
}

fn make_big_ball() {
    let mut plushie = examples::bigball();
    plushie.animate();
    save_mesh("generated/bigball.stl", plushie.to_mesh());
}

fn save_and_stuff_diamnond() {
    use Stitch::*;
    let p = Pattern {
        starting_circle: 4,
        ending_circle: 4,
        rounds: vec![vec![Sc, Inc, Sc, Sc], vec![Sc, Dec, Sc, Sc]],
    };
    let mut plushie = Plushie::from_pattern(p);

    save_mesh(
        "generated/from_pattern/before_stuffing.stl",
        plushie.to_mesh(),
    );
    plushie.animate();
    save_mesh(
        "generated/from_pattern/after_stuffing.stl",
        plushie.to_mesh(),
    );
    plushie.animate();
    save_mesh(
        "generated/from_pattern/after_stuffing_again.stl",
        plushie.to_mesh(),
    );
}
