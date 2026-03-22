use bevy::prelude::*;
use bevy_egui::input::{EguiWantsInput, egui_wants_any_input};
use std::sync::atomic::{AtomicBool, Ordering};

// TODO unify the names

#[derive(Resource, Default)]
pub struct UiUsedInput(AtomicBool);

pub fn reset(captured: Res<UiUsedInput>) {
    captured.0.store(false, Ordering::Relaxed);
}

impl UiUsedInput {
    pub fn capture(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn used(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

pub fn mark_input_as_captued_if_egui_wants_it(a: Res<UiUsedInput>, b: Res<EguiWantsInput>) {
    if egui_wants_any_input(b) {
        a.capture();
    }
}

pub fn world_input(ui_input: Res<UiUsedInput>) -> bool {
    return !ui_input.used();
}
