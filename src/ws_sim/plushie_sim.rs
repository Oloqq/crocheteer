use super::sim::{Data, Simulation};
use super::tokens::Tokens;
use crate::plushie::parse_to_any_plushie;
use crate::plushie::PlushieTrait;
use crate::{common::*, skeletonization, token_args};

use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum RunState {
    Paused,
    Running,
    RunningFor(usize),
}

#[derive(Clone)]
pub struct PlushieControls {
    need_init: bool,
    run_state: RunState,
}

impl PlushieControls {
    fn new() -> Self {
        Self {
            need_init: true,
            run_state: RunState::Paused,
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
            "key": "update",
            "data": {
                "points": self.plushie.nodes_to_json(),
                "centroids": self.plushie.centroids_to_json()
            }
        })
    }

    fn get_init_data(&self) -> JSON {
        if let Some(p) = &self.secondary_plushie {
            serde_json::json!({
                "key": "ini2",
                "data": serde_json::json!([self.plushie.init_data(), p.init_data()]),
            })
        } else {
            serde_json::json!({
                "key": "initialize",
                "data": self.plushie.init_data(),
            })
        }
    }

    fn change_pattern(&mut self, msg: &str) -> Result<(), String> {
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
                "data": data
            })
            .to_string(),
        )
    }

    fn react_internal(&mut self, msg: &str) -> Result<(), super::tokens::Error> {
        if msg.len() == 0 {
            log::info!("Empty message");
            return Ok(());
        }

        let tokens = Tokens::from(msg)?;
        log::info!("Message tokens: {tokens:?}");
        let command: &str = tokens.get(0)?;
        let controls = &mut self.controls;

        match command {
            "pos" => {
                let (id, x, y, z) = token_args!(tokens, usize, f32, f32, f32);
                self.plushie.set_point_position(id, Point::new(x, y, z));
            }
            "pattern" => match self.change_pattern(msg) {
                Ok(_) => {
                    self.controls.need_init = true;
                    self.send("status", "Loaded the pattern");
                }
                Err(error) => {
                    self.send("status", format!("Couldn't parse: {}", error).as_str());
                }
            },
            "pause" => controls.run_state = RunState::Paused,
            "resume" => controls.run_state = RunState::Running,
            "advance" => {
                controls.run_state = match controls.run_state {
                    RunState::RunningFor(steps) => RunState::RunningFor(steps + 1),
                    RunState::Paused => RunState::RunningFor(1),
                    RunState::Running => RunState::RunningFor(1),
                }
            }
            "setparams" => {
                let serialized = tokens.get(1)?;
                let deserd = serde_json::from_str(serialized).map_err(|e| {
                    log::error!("{e}");
                    super::tokens::Error::CantParseParams
                })?;
                self.plushie.set_params(deserd);
            }
            "getparams" => {
                let serialized = serde_json::to_string(self.plushie.params()).unwrap();
                self.send("params", &serialized);
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
                unimplemented!()
                // let plushie = crate::plushie::Pointcloud::from_points_str(tokens.get(1).unwrap());
                // self.plushie = Box::new(plushie);
                // self.controls.need_init = true;
                // self.send("status", "loaded pointcloud");
            }
            "calculate-normals" => {
                let normals =
                    skeletonization::local_surface_normals_per_point(self.plushie.get_points());

                self.send("normals", serde_json::to_string(&normals).unwrap().as_str());
            }
            "do-clustering" => {
                let plushie = self
                    .plushie
                    .as_animated()
                    .expect("This to be used with animated plushie");

                let colors: Vec<(usize, usize, usize)> = (0..plushie.nodes.colors.len())
                    .map(|_| (255, 255, 255))
                    .collect();

                self.send(
                    "change-colors",
                    serde_json::to_string(&colors).unwrap().as_str(),
                );
            }
            "initial-cross-sections" => {
                // let normals =
                //     skeletonization::local_surface_normals_per_point(self.plushie.get_points());

                // skeletonization::detect_initial_cross_sections();

                // self.send(
                //     "clusters",
                //     serde_json::to_string(&vec![Point::new(0.0, 1.0, 0.0)])
                //         .unwrap()
                //         .as_str(),
                // );

                // self.send(
                //     "seed-points",
                //     serde_json::to_string(&vec![Point::new(0.0, 1.0, 0.0)])
                //         .unwrap()
                //         .as_str(),
                // );
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

        let data = match self.controls.run_state {
            RunState::Paused => None,
            RunState::Running | RunState::RunningFor(_) => {
                self.plushie.step(dt);
                Some(self.get_update_data().to_string())
            }
        };

        self.controls.run_state = match self.controls.run_state {
            RunState::Paused => RunState::Paused,
            RunState::Running => RunState::Running,
            RunState::RunningFor(steps) => {
                if steps == 1 {
                    RunState::Paused
                } else {
                    RunState::RunningFor(steps - 1)
                }
            }
        };

        data
    }

    fn react(&mut self, msg: &str) {
        match self.react_internal(msg) {
            Ok(_) => (),
            Err(e) => log::error!("Message parsing error: {e:?} on message: {msg}"),
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
