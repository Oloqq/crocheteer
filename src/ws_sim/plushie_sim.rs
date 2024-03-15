use super::sim::{Data, Simulation};
use crate::common::*;
use crate::plushie::LegacyPlushie;
use crate::plushie::PlushieTrait;

use std::sync::{Arc, Mutex};

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

pub struct PlushieSimulation {
    controls: PlushieControls,
    plushie: Box<dyn PlushieTrait>,
    messages: Arc<Mutex<Vec<String>>>,
}

impl PlushieSimulation {
    pub fn from(plushie: impl PlushieTrait) -> Self {
        Self {
            controls: PlushieControls::new(),
            plushie: Box::new(plushie),
            messages: Arc::new(Mutex::new(vec![])),
        }
    }

    fn get_update_data(&self) -> JSON {
        serde_json::json!({
            "key": "upd",
            "dat": {
                "points": self.plushie.nodes_to_json(),
                "centroids": self.plushie.centroids_to_json()
            }
        })
    }

    fn get_init_data(&self) -> JSON {
        serde_json::json!({
            "key": "ini",
            "dat": self.plushie.whole_to_json()
        })
    }

    fn change_pattern(&mut self, msg: &str, _soft: bool) -> Result<(), String> {
        let (_, pattern) = match msg.split_once(" ") {
            Some(x) => x,
            None => return Err("frontend fuckup".into()),
        };
        log::info!("Changing pattern...");
        let new = LegacyPlushie::parse_any_format(pattern)?;
        // if soft {
        //     new.position_based_on(*self.plushie);
        // }
        self.plushie = Box::new(new);
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
            "pattern" | "soft_update" => {
                if let Err(error) = self.change_pattern(msg, command == "soft_update") {
                    self.send("status", format!("Couldn't parse: {}", error).as_str());
                } else {
                    self.controls.need_init = true;
                    self.send("status", "Loaded the pattern");
                }
            }
            "pause" => controls.paused = true,
            "resume" => controls.paused = false,
            "advance" => controls.advance += 1,
            "gravity" => self.plushie.params().gravity = tokens.get(1).unwrap().parse().unwrap(),
            "stuffing" => log::warn!("this should be removed from the frontend"),
            "centroid.amount" => {
                let num: usize = tokens.get(1).unwrap().parse().unwrap();
                self.plushie.params().centroids.number = num;
            }
            "load_example" => {
                use crate::plushie::examples;
                let name = tokens.get(1).unwrap();
                match examples::get(name) {
                    Some((pattern, plushie)) => {
                        self.controls.need_init = true;
                        self.plushie = Box::new(plushie);
                        self.send("pattern_update", &pattern.human_readable());
                        self.send("status", "Loaded an example");
                    }
                    None => self.send("status", "no such example"),
                }
            }
            _ => log::error!("Unexpected msg: {msg}"),
        }
    }

    fn clone(&self) -> Self {
        PlushieSimulation {
            controls: self.controls.clone(),
            plushie: self.plushie.clone(),
            messages: self.messages.clone(),
        }
    }
}
