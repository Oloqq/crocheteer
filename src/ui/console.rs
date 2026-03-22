use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self},
};
use egui_console::ConsoleEvent;

use crate::ui::{UiUsedInput, data::ConsoleState, utils::using_resizer_bottom};

pub fn console_window(
    mut state: ResMut<ConsoleState>,
    mut contexts: EguiContexts,
    captured: Res<UiUsedInput>,
) -> Result {
    if !state.visible {
        return Ok(());
    }

    let ctx = contexts.ctx_mut()?;
    let panel_id = egui::Id::new("console");

    egui::TopBottomPanel::bottom(panel_id)
        .resizable(true)
        .show(ctx, |ui| {
            let console_response = state.console.draw(ui);
            if let ConsoleEvent::Command(command) = console_response {
                state.console.write(&command);
                state.console.prompt();
            }
        });

    // prevent world events on resizing
    if using_resizer_bottom(ctx, panel_id) {
        captured.capture();
    }

    Ok(())
}
