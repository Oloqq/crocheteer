use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self},
};
use crossbeam_channel::{Receiver, Sender};
use egui_console::ConsoleEvent;

use crate::ui::{UiUsedInput, data::ConsoleState, utils::using_resizer_bottom};

pub fn console_window(
    mut state: ResMut<ConsoleState>,
    mut contexts: EguiContexts,
    ui_used_input: Res<UiUsedInput>,
    console_receiver: Res<ConsoleReceiver>,
) -> Result {
    while let Ok(message) = console_receiver.0.try_recv() {
        state.console.write(&message.text);
    }

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
        ui_used_input.set_true();
    }

    Ok(())
}

#[derive(Resource)]
pub struct ConsolePipe {
    pub sender: Sender<ConsoleMessage>,
}

#[derive(Resource)]
pub struct ConsoleReceiver(pub Receiver<ConsoleMessage>);

pub struct ConsoleMessage {
    pub text: String,
}
