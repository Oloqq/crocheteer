mod code_editor;
mod control_panel;
mod data;
mod input_capture;
mod menu_bar;
mod utils;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
pub use input_capture::UiUsedInput;

use crate::ui::{
    code_editor::code_editor_ui,
    control_panel::ui_example_system,
    data::{CodeEditorState, UiState},
    input_capture::mark_input_as_captued_if_egui_wants_it,
    menu_bar::top_panel,
};

pub use input_capture::world_input;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.init_resource::<CodeEditorState>();
        app.init_resource::<UiState>();
        app.init_resource::<UiUsedInput>();
        app.add_systems(Startup, set_style);
        app.add_systems(
            EguiPrimaryContextPass,
            (
                (input_capture::reset),
                top_panel,
                (ui_example_system, code_editor_ui),
                mark_input_as_captued_if_egui_wants_it,
            )
                .chain(),
        );
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
