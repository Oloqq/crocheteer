// #![allow(unused)]

// use super::sim::{Data, Simulation};

// #[derive(Clone)]
// pub struct BallControls {
//     paused: bool,
// }

// impl BallControls {
//     fn new() -> Self {
//         Self { paused: false }
//     }
// }

// #[derive(Clone)]
// pub struct BallSimulation {
//     controls: BallControls,
//     ball_pos: [f32; 3],
//     dy: f32,
// }

// impl BallSimulation {
//     pub fn new() -> Self {
//         Self {
//             ball_pos: [1.0, 0.5, 0.0],
//             dy: 1.0,
//             controls: BallControls::new(),
//         }
//     }

//     fn update(&mut self, dt: f32) {
//         self.ball_pos[1] += 0.1 * self.dy * dt;
//         if self.ball_pos[1] > 5.0 {
//             self.dy = -1.0;
//         } else if self.ball_pos[1] < 0.0 {
//             self.dy = 1.0;
//         }
//     }

//     fn get_data(&self) -> [f32; 3] {
//         self.ball_pos
//     }
// }

// impl Simulation for BallSimulation {
//     fn step(&mut self, dt: f32) -> Option<Data> {
//         if self.controls.paused {
//             None
//         } else {
//             self.update(dt);

//             Some(serde_json::json!(self.get_data()).to_string())
//         }
//     }

//     fn react(&mut self, msg: &str) {
//         let controls = &mut self.controls;
//         match msg {
//             "pause" => controls.paused = true,
//             "resume" => controls.paused = false,
//             _ => println!("Unexpected msg: {msg}"),
//         }
//     }
// }
