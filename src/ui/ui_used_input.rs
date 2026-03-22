use bevy::prelude::*;
use bevy_egui::input::{EguiWantsInput, egui_wants_any_input};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Resource, Default)]
pub struct UiUsedInput(AtomicBool);

pub fn reset(captured: Res<UiUsedInput>) {
    captured.0.store(false, Ordering::Relaxed);
}

impl UiUsedInput {
    pub fn set_true(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn get(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

pub fn adjust_to_egui_wants_input(a: Res<UiUsedInput>, b: Res<EguiWantsInput>) {
    if egui_wants_any_input(b) {
        a.set_true();
    }
}

pub fn world_input(ui_used_input: Res<UiUsedInput>) -> bool {
    return !ui_used_input.get();
}
