use super::sim::{Data, Simulation};
use super::tokens::Tokens;
use crate::plushie::parse_to_any_plushie;
use crate::plushie::PlushieTrait;
use crate::{common::*, token_args};

use std::fs;
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
    secondary_plushie: Option<Box<dyn PlushieTrait>>,
    messages: Arc<Mutex<Vec<String>>>,
}

impl PlushieSimulation {
    pub fn from(plushie: impl PlushieTrait) -> Self {
        Self {
            controls: PlushieControls::new(),
            plushie: Box::new(plushie),
            secondary_plushie: None,
            messages: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn with_secondary(plushie: impl PlushieTrait, secondary: impl PlushieTrait) -> Self {
        let mut res = Self::from(plushie);
        res.secondary_plushie = Some(Box::new(secondary));
        res
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
        if let Some(p) = &self.secondary_plushie {
            serde_json::json!({
                "key": "ini2",
                "dat": serde_json::json!([self.plushie.init_data(), p.init_data()]),
            })
        } else {
            serde_json::json!({
                "key": "ini",
                "dat": self.plushie.init_data(),
            })
        }
    }

    fn change_pattern(&mut self, msg: &str, _soft: bool) -> Result<(), String> {
        log::info!("Changing pattern...");

        let (_, version_pattern) = msg.split_once(" ").ok_or("frontend fuckup")?;
        let (selector, pattern) = version_pattern.split_once(" ").ok_or("frontend fuckup")?;

        self.plushie = parse_to_any_plushie(selector, pattern)?;
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

    fn react_internal(&mut self, msg: &str) -> Result<(), super::tokens::Error> {
        let tokens = Tokens::from(msg)?;
        log::trace!("Message tokens: {tokens:?}");
        let command: &str = tokens.get(0)?;
        let controls = &mut self.controls;

        match command {
            "pos" => {
                let (id, x, y, z) = token_args!(tokens, usize, f32, f32, f32);
                self.plushie.set_point_position(id, Point::new(x, y, z));
            }
            "pattern" | "soft_update" => match self.change_pattern(msg, command == "soft_update") {
                Ok(_) => {
                    self.controls.need_init = true;
                    self.send("status", "Loaded the pattern");
                }
                Err(error) => {
                    self.send("status", format!("Couldn't parse: {}", error).as_str());
                }
            },
            "pause" => controls.paused = true,
            "resume" => controls.paused = false,
            "advance" => controls.advance += 1,
            "stuffing" => log::warn!("this should be removed from the frontend"),
            "setparams" => {
                let serialized = tokens.get(1)?;
                let deserd = serde_json::from_str(serialized)
                    .map_err(|_| super::tokens::Error::CantParseParams)?;
                self.plushie.set_params(deserd);
            }
            "getparams" => {
                let serialized = serde_json::to_string(self.plushie.params()).unwrap();
                self.send("params", &serialized);
            }
            "load_example" => {
                self.send("status", "examples are temporarily not available");
                log::warn!("examples are temporarily not available");
                // let name = tokens.get(1).unwrap();
                // match examples::get(name) {
                //     Some((pattern, plushie)) => {
                //         self.controls.need_init = true;
                //         self.plushie = Box::new(plushie);
                //         self.send("pattern_update", &pattern.human_readable());
                //         self.send("status", "Loaded an example");
                //     }
                //     None => self.send("status", "no such example"),
                // }
            }
            "save" => {
                if let Ok(name) = tokens.get(1) {
                    fs::write(
                        format!("generated/nodes/{name}.json").as_str(),
                        serde_json::to_string(&self.plushie.nodes_to_json()).unwrap(),
                    )
                    .unwrap();
                } else {
                    self.send("status", "provide a name");
                }
            }
            "export-pointcloud" => self.send("export", &self.plushie.nodes_to_json().to_string()),
            "import-pointcloud" => {
                let plushie = crate::plushie::Pointcloud::from_points_str(tokens.get(1).unwrap());
                self.plushie = Box::new(plushie);
                self.controls.need_init = true;
                self.send("status", "loaded pointcloud");
            }
            _ => log::error!("Unexpected msg: {msg}"),
        };
        Ok(())
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
        match self.react_internal(msg) {
            Ok(_) => (),
            Err(e) => log::warn!("Message parsing error: {e:?} on message: {msg}"),
        };
    }

    fn clone(&self) -> Self {
        let secondary_plushie: Option<Box<dyn PlushieTrait>> =
            if let Some(refbox) = &self.secondary_plushie {
                Some(refbox.clonebox())
            } else {
                None
            };
        PlushieSimulation {
            controls: self.controls.clone(),
            plushie: self.plushie.clonebox(),
            messages: self.messages.clone(),
            secondary_plushie,
        }
    }
}
