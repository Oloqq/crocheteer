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
use pattern::{construction::PatternBuilder, Pattern};
use plushie::Plushie;
use ws_sim::ball_sim::BallSimulation;
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
        let pattern = PatternBuilder::new(6).full_rounds(4).build().unwrap();
        let plushie = Plushie::from_pattern(pattern);
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
    let pattern = PatternBuilder::new(6).full_rounds(4).build().unwrap();
    // println!("{pattern:?}");
    let mut plushie = Plushie::from_pattern(pattern);
    plushie.animate();
    save_mesh("generated/pillar.stl", plushie.to_mesh());
}

fn make_ball() {
    use Stitch::*;
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .full_rounds(1)
        .round_like(&vec![Dec])
        .build()
        .unwrap();
    // println!("{pattern:?}");
    let mut plushie = Plushie::from_pattern(pattern);
    plushie.animate();
    save_mesh("generated/ball.stl", plushie.to_mesh());
}

fn make_big_ball() {
    use Stitch::*;
    let pattern = PatternBuilder::new(6)
        .round_like(&vec![Inc])
        .round_like(&vec![Sc, Inc])
        .full_rounds(1)
        .round_like(&vec![Sc, Dec])
        .round_like(&vec![Dec])
        .build()
        .unwrap();
    // println!("{pattern:?}");
    let mut plushie = Plushie::from_pattern(pattern);
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
