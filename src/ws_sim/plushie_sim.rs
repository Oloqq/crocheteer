use std::{
    fs,
    sync::{Arc, Mutex},
};

use serde_json::json;

use super::{
    sim::{Data, Simulation},
    tokens::Tokens,
};
use crate::{
    common::*,
    plushie::{parse_to_any_plushie, PlushieTrait},
    skeletonization, token_args,
};

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

    #[allow(unused)]
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

        // let (_, version_pattern) = msg.split_once(" ").ok_or("frontend fuckup")?;
        // let (selector, pattern) = version_pattern.split_once(" ").ok_or("frontend fuckup")?;
        let selector = "flow";
        let (_, pattern) = msg.split_once(" ").ok_or("frontend fuckup")?;

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
            "move" => {
                let (kind, id, x, y, z) = token_args!(tokens, String, usize, f32, f32, f32);
                match kind.as_str() {
                    "node" => self.plushie.set_point_position(id, Point::new(x, y, z)),
                    _ => log::error!("Unexpected kind of thing to move"),
                }
            }
            "pattern" => match self.change_pattern(msg) {
                Ok(_) => {
                    self.controls.need_init = true;
                    self.send("status", "Loaded the pattern");
                    let serialized = serde_json::to_string(self.plushie.params()).unwrap();
                    self.send("params", &serialized);
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
                let bones = self.plushie.params().skelet_stuffing.bones.clone();
                self.plushie.set_params(deserd);
                self.plushie.params_mut().skelet_stuffing.bones = bones;
            }
            "getparams" => {
                let serialized = serde_json::to_string(self.plushie.params_mut()).unwrap();
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
            "getperf" => {
                if self.plushie.params().track_performance {
                    self.send(
                        "perfdata",
                        serde_json::to_string(&self.plushie.as_animated().unwrap().perf)
                            .unwrap()
                            .as_str(),
                    );
                } else {
                    self.send("status", "performance not tracked");
                }
            }
            "import-pointcloud" => {
                unimplemented!()
                // let plushie = crate::plushie::Pointcloud::from_points_str(tokens.get(1).unwrap());
                // self.plushie = Box::new(plushie);
                // self.controls.need_init = true;
                // self.send("status", "loaded pointcloud");
            }
            "newskelet" => {
                let params = &self.plushie.params().skelet_stuffing;
                if !params.enable {
                    println!("enable skeletonization first");
                    return Ok(());
                }

                let bones = skeletonization::get_skelet(
                    &self.plushie.as_animated().unwrap(),
                    params.cluster_number,
                    params.must_include_points,
                    params.allowed_overlap,
                    &mut None,
                );

                self.plushie.params_mut().skelet_stuffing.bones = bones;
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

                const CLUSTER_NUM: usize = 4;
                let cluster_colors: [(usize, usize, usize); CLUSTER_NUM] =
                    [(255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 0)];
                let (cluster_membership, centroids) =
                    skeletonization::do_clustering(CLUSTER_NUM, &plushie.nodes.points);

                let seeds = skeletonization::select_seeds(
                    &plushie.nodes.points,
                    &cluster_membership,
                    &centroids,
                );

                let colors: Vec<(usize, usize, usize)> = {
                    let mut colors: Vec<(usize, usize, usize)> = cluster_membership
                        .iter()
                        .map(|cluster| cluster_colors[*cluster])
                        .collect();
                    for seed in &seeds {
                        colors[*seed] = (255, 255, 255);
                    }
                    colors
                };

                self.send(
                    "change-colors",
                    serde_json::to_string(&json!({
                        "standard": &colors,
                    }))
                    .unwrap()
                    .as_str(),
                );

                self.send(
                    "change-centroids",
                    serde_json::to_string(&json!({
                        "centroids": &centroids,
                        "colors": &cluster_colors,
                    }))
                    .unwrap()
                    .as_str(),
                );
            }
            "initial-cross-sections" => {
                const CLUSTER_NUM: usize = 4;
                let cloud = &self.plushie.as_animated().unwrap().nodes.points;
                let connectivity =
                    skeletonization::Connectivity::new(&self.plushie.as_animated().unwrap().edges);
                let surface_normals = skeletonization::local_surface_normals_per_point(cloud);
                let cross_sections = skeletonization::detect_initial_cross_sections(
                    cloud,
                    &connectivity,
                    CLUSTER_NUM,
                    &surface_normals,
                );

                let seeds: Vec<usize> = cross_sections.iter().map(|cs| cs.seed).collect();
                let angles: Vec<(f32, f32)> = cross_sections
                    .iter()
                    .map(|cs| (cs.normal.0, cs.normal.1))
                    .collect();
                let inliers: Vec<Vec<usize>> =
                    cross_sections.iter().map(|cs| cs.inliers.clone()).collect();

                let cluster_colors: [(usize, usize, usize); CLUSTER_NUM] =
                    [(255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 0)];
                let colors: Vec<(usize, usize, usize)> = (0..cloud.len())
                    .map(|i| {
                        for (ci, color) in cluster_colors.iter().enumerate() {
                            if inliers[ci].contains(&i) {
                                return color.clone();
                            }
                        }
                        return (255, 255, 255);
                    })
                    .collect();

                let all_white = vec![(255, 255, 255); cloud.len()];
                let mut variable_colors: Vec<Vec<(usize, usize, usize)>> =
                    vec![all_white.clone(); inliers.len()];
                for (i, plane_inliers) in inliers.iter().enumerate() {
                    for point in plane_inliers {
                        variable_colors[i][*point] = cluster_colors[i];
                    }
                }

                self.send(
                    "change-colors",
                    serde_json::to_string(&json!({
                        "standard": &colors,
                        "variable": &variable_colors,
                    }))
                    .unwrap()
                    .as_str(),
                );

                let seed_points: Vec<Point> = seeds.iter().map(|s| cloud[*s].clone()).collect();
                self.send(
                    "change-centroids",
                    serde_json::to_string(&json!({
                        "centroids": &seed_points,
                        "colors": &cluster_colors,
                        "angles": &angles
                    }))
                    .unwrap()
                    .as_str(),
                );
            }
            "growing" => {
                const CLUSTER_NUM: usize = 50;
                let cloud = &self.plushie.as_animated().unwrap().nodes.points;
                let connectivity =
                    skeletonization::Connectivity::new(&self.plushie.as_animated().unwrap().edges);
                let surface_normals = skeletonization::local_surface_normals_per_point(cloud);
                let cross_sections = skeletonization::detect_initial_cross_sections(
                    cloud,
                    &connectivity,
                    CLUSTER_NUM,
                    &surface_normals,
                );
                let parts: Vec<skeletonization::Part> =
                    skeletonization::grow(cloud, &connectivity, cross_sections, &surface_normals);
                println!(
                    "parts: {}, sections: {}",
                    parts.len(),
                    parts.iter().flat_map(|p| &p.sections).count()
                );

                let all_white = vec![(255, 255, 255); cloud.len()];
                let highlight_color = (255, 0, 0);

                let mut variable_node_colors: Vec<Vec<(usize, usize, usize)>> =
                    vec![all_white.clone(); parts.len()];
                for (partnum, part) in parts.iter().enumerate() {
                    for section in &part.sections {
                        for point in &section.inliers {
                            variable_node_colors[partnum][*point] = highlight_color.clone();
                        }
                    }
                }

                self.send(
                    "change-colors",
                    serde_json::to_string(&json!({
                        "standard": &all_white,
                        "variable": &variable_node_colors,
                    }))
                    .unwrap()
                    .as_str(),
                );

                let mut skeleton: Vec<Point> = Vec::new();
                let mut colors: Vec<(usize, usize, usize)> = Vec::new();
                let mut part_to_centroids: Vec<Vec<usize>> = vec![Vec::new(); parts.len()];
                let mut centroidnum = 0;
                for (partnum, part) in parts.iter().enumerate() {
                    for section in &part.sections {
                        skeleton.push(Point::from(section.center));
                        colors.push((255, 165, 255));
                        part_to_centroids[partnum].push(centroidnum);
                        centroidnum += 1;
                    }
                }

                self.send(
                    "change-centroids",
                    serde_json::to_string(&json!({
                        "centroids": &skeleton,
                        "colors": &colors,
                        "variable": &part_to_centroids
                    }))
                    .unwrap()
                    .as_str(),
                );
            }
            "cost" => {
                const CLUSTER_NUM: usize = 50;
                let cloud = &self.plushie.as_animated().unwrap().nodes.points;
                let connectivity =
                    skeletonization::Connectivity::new(&self.plushie.as_animated().unwrap().edges);
                let surface_normals = skeletonization::local_surface_normals_per_point(cloud);
                let cross_sections = skeletonization::detect_initial_cross_sections(
                    cloud,
                    &connectivity,
                    CLUSTER_NUM,
                    &surface_normals,
                );
                let parts: Vec<skeletonization::Part> =
                    skeletonization::grow(cloud, &connectivity, cross_sections, &surface_normals);
                let (parts, costs) = skeletonization::sort_by_cost(parts, cloud);
                println!("parts: {}", parts.len());
                println!("costs: {:?}", costs);

                let all_white = vec![(255, 255, 255); cloud.len()];
                let highlight_color = (255, 0, 0);

                let mut variable_node_colors: Vec<Vec<(usize, usize, usize)>> =
                    vec![all_white.clone(); parts.len()];
                for (partnum, part) in parts.iter().enumerate() {
                    for section in &part.sections {
                        for point in &section.inliers {
                            variable_node_colors[partnum][*point] = highlight_color.clone();
                        }
                    }
                }

                self.send(
                    "change-colors",
                    serde_json::to_string(&json!({
                        "standard": &all_white,
                        "variable": &variable_node_colors,
                    }))
                    .unwrap()
                    .as_str(),
                );

                let mut skeleton: Vec<Point> = Vec::new();
                let mut colors: Vec<(usize, usize, usize)> = Vec::new();
                let mut part_to_centroids: Vec<Vec<usize>> = vec![Vec::new(); parts.len()];
                let mut centroidnum = 0;
                for (partnum, part) in parts.iter().enumerate() {
                    for section in &part.sections {
                        skeleton.push(Point::from(section.center));
                        colors.push((255, 165, 255));
                        part_to_centroids[partnum].push(centroidnum);
                        centroidnum += 1;
                    }
                }

                self.send(
                    "change-centroids",
                    serde_json::to_string(&json!({
                        "centroids": &skeleton,
                        "colors": &colors,
                        "variable": &part_to_centroids
                    }))
                    .unwrap()
                    .as_str(),
                );
            }
            "optimparts" => {
                println!("selecting parts");
                const CLUSTER_NUM: usize = 50;
                let cloud = &self.plushie.as_animated().unwrap().nodes.points;
                let connectivity =
                    skeletonization::Connectivity::new(&self.plushie.as_animated().unwrap().edges);
                let surface_normals = skeletonization::local_surface_normals_per_point(cloud);
                let cross_sections = skeletonization::detect_initial_cross_sections(
                    cloud,
                    &connectivity,
                    CLUSTER_NUM,
                    &surface_normals,
                );
                let parts: Vec<skeletonization::Part> =
                    skeletonization::grow(cloud, &connectivity, cross_sections, &surface_normals);
                println!("all parts: {}", parts.len());
                let parts = skeletonization::select_parts(
                    parts,
                    skeletonization::PartSelectionParams::new(cloud.len(), 0.95, 5.0),
                    cloud,
                );
                println!("selected parts: {}", parts.len());

                let all_white = vec![(255, 255, 255); cloud.len()];
                let highlight_color = (255, 0, 0);

                let mut variable_node_colors: Vec<Vec<(usize, usize, usize)>> =
                    vec![all_white.clone(); parts.len()];
                for (partnum, part) in parts.iter().enumerate() {
                    for section in &part.sections {
                        for point in &section.inliers {
                            variable_node_colors[partnum][*point] = highlight_color.clone();
                        }
                    }
                }

                self.send(
                    "change-colors",
                    serde_json::to_string(&json!({
                        "standard": &all_white,
                        "variable": &variable_node_colors,
                    }))
                    .unwrap()
                    .as_str(),
                );

                let mut skeleton: Vec<Point> = Vec::new();
                let mut colors: Vec<(usize, usize, usize)> = Vec::new();
                let mut part_to_centroids: Vec<Vec<usize>> = vec![Vec::new(); parts.len()];
                let mut centroidnum = 0;
                for (partnum, part) in parts.iter().enumerate() {
                    for section in &part.sections {
                        skeleton.push(Point::from(section.center));
                        colors.push((255, 165, 255));
                        part_to_centroids[partnum].push(centroidnum);
                        centroidnum += 1;
                    }
                }

                self.send(
                    "change-centroids",
                    serde_json::to_string(&json!({
                        "centroids": &skeleton,
                        "colors": &colors,
                        "variable": &part_to_centroids
                    }))
                    .unwrap()
                    .as_str(),
                );
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
