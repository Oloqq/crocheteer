use crate::common::*;
#[allow(unused)]
use crate::meshes_sandbox::*;
use crate::pattern::Stitch;

extern crate nalgebra as na;

mod args;
mod common;
mod meshes_sandbox;
mod pattern;
mod plushie;

use args::*;
use pattern::Pattern;
use plushie::Plushie;

fn main() {
    let args = Args::from_args();
    if let Some(num) = args.dev {
        exec_dev_action(num);
        return;
    }

    if let Some(pattern_path) = args.show {
        let pattern = Pattern::from_file(pattern_path);
        let mut plushie = Plushie::from_pattern(pattern);
        if args.verbose {
            save_mesh("generated/before_stuffing.stl", plushie.to_mesh());
        }
        plushie.stuff();
        if args.verbose {
            save_mesh("generated/after_stuffing.stl", plushie.to_mesh());
        }
        save_mesh(args.output.to_str().unwrap(), plushie.to_mesh());
    }

    // let mut plushie = diamond_plushie_direct();
}

fn exec_dev_action(num: usize) {
    println!("dev action {num}");
    match num {
        1 => save_and_stuff_diamnond(),
        _ => println!("no such action"),
    }
}

fn save_and_stuff_diamnond() {
    use Stitch::*;
    let p = Pattern {
        starting_circle: 4,
        ending_circle: 4,
        rounds: vec![
            vec![Single, Increase, Single, Single],
            vec![Single, Decrease, Single, Single],
        ],
    };
    let mut plushie = Plushie::from_pattern(p);

    save_mesh(
        "generated/from_pattern/before_stuffing.stl",
        plushie.to_mesh(),
    );
    plushie.stuff();
    save_mesh(
        "generated/from_pattern/after_stuffing.stl",
        plushie.to_mesh(),
    );
    plushie.stuff();
    save_mesh(
        "generated/from_pattern/after_stuffing_again.stl",
        plushie.to_mesh(),
    );
}
