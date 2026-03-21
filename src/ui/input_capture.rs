use bevy::prelude::*;
use bevy_egui::input::{EguiWantsInput, egui_wants_any_input};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Resource, Default)]
pub struct InputCaptured(AtomicBool);

pub fn reset(captured: Res<InputCaptured>) {
    captured.0.store(false, Ordering::Relaxed);
}

impl InputCaptured {
    pub fn capture(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
}

pub fn world_input(
    captured: Option<Res<InputCaptured>>,
    wants_input: Option<Res<EguiWantsInput>>,
) -> bool {
    let relevant_to_ui = wants_input.is_some_and(|w| egui_wants_any_input(w))
        || captured.is_some_and(|n| n.0.load(Ordering::Relaxed));
    return !relevant_to_ui;
}
