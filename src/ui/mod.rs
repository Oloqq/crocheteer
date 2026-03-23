mod code_editor;
mod console;
mod control_panel;
mod data;
mod menu_bar;
mod ui_used_input;
mod utils;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
pub use console::{ConsoleMessage, ConsolePipe};
pub use ui_used_input::UiUsedInput;

use crate::{
    plushie::BuildPlushieFromPattern,
    ui::{
        code_editor::code_editor_ui,
        console::{ConsoleReceiver, console_window},
        control_panel::control_panel,
        data::{CodeEditorState, ConsoleState, UiState},
        menu_bar::top_panel,
    },
};

pub use ui_used_input::world_input;

pub struct UiPlugin {
    pub initial_pattern: String,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.init_resource::<UiState>();
        app.insert_resource(CodeEditorState {
            code: self.initial_pattern.clone(),
            ..default()
        });
        app.init_resource::<ConsoleState>();
        app.init_resource::<UiUsedInput>();
        app.add_systems(Startup, (set_style, build_initial_plushie));
        app.add_systems(
            EguiPrimaryContextPass,
            (
                ui_used_input::reset,
                top_panel,
                (control_panel, code_editor_ui),
                console_window,
                ui_used_input::adjust_to_egui_wants_input,
            )
                .chain(),
        );

        let (tx, rx) = crossbeam_channel::unbounded::<ConsoleMessage>();
        app.insert_resource(ConsolePipe { sender: tx });
        app.insert_resource(ConsoleReceiver(rx));
    }
}

fn set_style(mut contexts: bevy_egui::EguiContexts) -> Result {
    use bevy_egui::egui;
    use bevy_egui::egui::style::ScrollStyle;

    let ctx = contexts.ctx_mut()?;
    ctx.set_visuals(egui::Visuals {
        window_corner_radius: 0.0.into(),
        ..Default::default()
    });
    ctx.style_mut(|style| {
        style.animation_time = 0.05;
        style.interaction.interact_radius = 0.0;
        style.spacing.scroll = ScrollStyle::solid();
    });
    Ok(())
}

fn build_initial_plushie(
    mut msgw: MessageWriter<BuildPlushieFromPattern>,
    state: Res<CodeEditorState>,
) {
    // msgw.write(BuildPlushieFromPattern {
    //     pattern: state.code.clone(),
    // });
}
