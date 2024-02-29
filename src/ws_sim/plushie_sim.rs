use crate::{common::Point, genetic::common::Program, plushie::Plushie};

use super::sim::{Data, Simulation};

#[derive(Clone)]
pub struct PlushieControls {
    paused: bool,
    advance: usize,
    need_init: bool,
}

impl PlushieControls {
    fn new() -> Self {
        Self {
            paused: true,
            advance: 0,
            need_init: true,
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

    fn get_update_data(&self) -> serde_json::Value {
        serde_json::json!({
            "key": "upd",
            "dat": serde_json::json!(&self.plushie.points)
        })
    }

    fn get_init_data(&self) -> serde_json::Value {
        serde_json::json!({
            "key": "ini",
            "dat": serde_json::json!(self.plushie)
        })
    }
}

impl Simulation for PlushieSimulation {
    fn step(&mut self, dt: f32) -> Option<Data> {
        if self.controls.need_init {
            self.controls.need_init = false;
            return Some(self.get_init_data().to_string());
        }

        if self.controls.paused && self.controls.advance == 0 {
            None
        } else {
            if self.controls.advance > 0 {
                self.controls.advance -= 1;
            }

            self.plushie.step(dt);

            let serialized = self.get_update_data().to_string();
            // println!("serialized: {serialized}");
            Some(serialized)
        }
    }

    fn react(&mut self, msg: &str) {
        let controls = &mut self.controls;
        if msg.starts_with("pos") {
            let tokens: Vec<&str> = msg.split(" ").collect();
            assert!(tokens.len() == 5);
            let id: usize = tokens[1].parse().unwrap();
            let x: f32 = tokens[2].parse().unwrap();
            let y: f32 = tokens[3].parse().unwrap();
            let z: f32 = tokens[4].parse().unwrap();
            self.plushie.points[id] = Point::new(x, y, z);
        } else if msg.starts_with("pattern") {
            let (_, pattern) = msg.split_once(" ").unwrap();
            let stitches = Program::deserialize(pattern).unwrap().tokens;
            self.plushie = Plushie::from_genetic(&(6, &stitches));
        } else {
            match msg {
                "pause" => controls.paused = true,
                "resume" => controls.paused = false,
                "advance" => controls.advance += 1,
                _ => println!("Unexpected msg: {msg}"),
            }
        }
    }
}
