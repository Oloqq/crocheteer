use crate::{common::Point, plushie::Plushie};

use super::sim::{Data, Simulation};

#[derive(Clone)]
pub struct PlushieControls {
    paused: bool,
    advance: usize,
}

impl PlushieControls {
    fn new() -> Self {
        Self {
            paused: true,
            advance: 1,
        }
    }
}

#[derive(Clone)]
pub struct PlushieSimulation {
    controls: PlushieControls,
    plushie: Plushie,
}

impl PlushieSimulation {
    pub fn from(plushie: Plushie) -> Self {
        Self {
            controls: PlushieControls::new(),
            plushie,
        }
    }

    fn get_data(&self) -> &Vec<Point> {
        &self.plushie.points
    }
}

impl Simulation for PlushieSimulation {
    fn step(&mut self, dt: f32) -> Option<Data> {
        if self.controls.paused && self.controls.advance == 0 {
            None
        } else {
            if self.controls.advance > 0 {
                self.controls.advance -= 1;
            }

            self.plushie.step(dt);

            let serialized = serde_json::json!(self.get_data()).to_string();
            // println!("serialized: {serialized}");
            Some(serialized)
        }
    }

    fn react(&mut self, msg: &str) {
        let controls = &mut self.controls;
        match msg {
            "pause" => controls.paused = true,
            "resume" => controls.paused = false,
            "advance" => controls.advance += 1,
            _ => println!("Unexpected msg: {msg}"),
        }
    }
}
