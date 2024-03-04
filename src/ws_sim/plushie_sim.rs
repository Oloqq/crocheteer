use std::sync::{Arc, Mutex};

use crate::{
    common::Point,
    genetic::common::Program,
    pattern::Pattern,
    plushie::{Plushie, Stuffing},
};

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
    messages: Arc<Mutex<Vec<String>>>,
}

impl PlushieSimulation {
    pub fn from(plushie: Plushie) -> Self {
        Self {
            controls: PlushieControls::new(),
            plushie,
            messages: Arc::new(Mutex::new(vec![])),
        }
    }

    fn get_update_data(&self) -> serde_json::Value {
        serde_json::json!({
            "key": "upd",
            "dat": {
                "points": serde_json::json!(&self.plushie.get_points_vec()),
                "centroids": self.plushie.centroids
            }
        })
    }

    fn get_init_data(&self) -> serde_json::Value {
        serde_json::json!({
            "key": "ini",
            "dat": serde_json::json!(self.plushie)
        })
    }

    fn change_pattern(&mut self, msg: &str) -> Result<(), String> {
        let (_, pattern) = match msg.split_once(" ") {
            Some(x) => x,
            None => return Err("frontend fuckup".into()),
        };
        log::info!("Changing pattern...");
        self.plushie = match Program::deserialize(pattern) {
            Ok(program) => Plushie::from_genetic(&(6, &program.tokens)),
            Err(e) => {
                log::info!("The pattern could not be interpreted as genetic");
                match Pattern::from_human_readable(pattern) {
                    Ok(pattern) => Plushie::from_pattern(pattern),
                    Err(e) => {
                        log::info!("The pattern could not be interpreted as human readable");
                        return Err(e);
                    }
                }
            }
        };
        Ok(())
    }

    fn send(&self, key: &str, data: &str) {
        self.messages.lock().unwrap().push(
            serde_json::json!({
                "key": key,
                "dat": data
            })
            .to_string(),
        )
    }
}

impl Simulation for PlushieSimulation {
    fn messages(&self) -> Arc<Mutex<Vec<String>>> {
        self.messages.clone()
    }

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
            Some(serialized)
        }
    }

    fn react(&mut self, msg: &str) {
        let controls = &mut self.controls;
        let tokens: Vec<&str> = msg.split(" ").collect();
        let command = *match tokens.get(0) {
            Some(command) => command,
            None => {
                log::error!("Unexpected msg: {msg}");
                return;
            }
        };

        log::info!("Message: {tokens:?}");
        match command {
            "pos" => {
                assert!(tokens.len() == 5);
                let id: usize = tokens[1].parse().unwrap();
                let x: f32 = tokens[2].parse().unwrap();
                let y: f32 = tokens[3].parse().unwrap();
                let z: f32 = tokens[4].parse().unwrap();
                self.plushie.set_point_position(id, Point::new(x, y, z));
            }
            "pattern" => {
                if let Err(error) = self.change_pattern(msg) {
                    self.send("status", format!("Couldn't parse: {}", error).as_str());
                } else {
                    self.controls.need_init = true;
                    self.send("status", "success");
                }
            }
            "pause" => controls.paused = true,
            "resume" => controls.paused = false,
            "advance" => controls.advance += 1,
            "gravity" => self.plushie.gravity = tokens.get(1).unwrap().parse().unwrap(),
            "stuffing" => {
                let name = tokens.get(1).unwrap();
                if let Some(stuffing) = match *name {
                    "None" => Some(Stuffing::None),
                    "PerRound" => Some(Stuffing::PerRound),
                    "Centroids" => Some(Stuffing::Centroids),
                    _ => {
                        log::error!("Unexpected stuffing: {name}");
                        None
                    }
                } {
                    self.plushie.stuffing = stuffing;
                };
            }
            "centroid.amount" => {
                if let Stuffing::Centroids = self.plushie.stuffing {
                    let num: usize = tokens.get(1).unwrap().parse().unwrap();
                    self.plushie.set_centroid_num(num);
                }
            }
            _ => log::error!("Unexpected msg: {msg}"),
        }
    }
}
