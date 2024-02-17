use crate::plushie::Plushie;

use super::sim::{Data, Simulation};

#[derive(Clone)]
pub struct PlushieControls {
    paused: bool,
}

impl PlushieControls {
    fn new() -> Self {
        Self { paused: false }
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

    fn update(&mut self, dt: f32) {
        // self.plushie
    }

    fn get_data(&self) -> [f32; 3] {
        // self.ball_pos
        todo!();
    }
}

impl Simulation for PlushieSimulation {
    fn step(&mut self, dt: f32) -> Option<Data> {
        if self.controls.paused {
            None
        } else {
            self.update(dt);

            Some(serde_json::json!(self.get_data()).to_string())
        }
    }

    fn react(&mut self, msg: &str) {
        let controls = &mut self.controls;
        match msg {
            "pause" => controls.paused = true,
            "resume" => controls.paused = false,
            _ => println!("Unexpected msg: {msg}"),
        }
    }
}
