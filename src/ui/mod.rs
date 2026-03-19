mod code_editor;
mod control_panel;
mod data;
mod input_capture;
mod menu_bar;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use input_capture::InputCaptured;

use crate::ui::{
    code_editor::code_editor_ui,
    control_panel::{configure_ui_state_system, configure_visuals_system, ui_example_system},
    data::{CodeEditorState, UiState},
    menu_bar::top_panel,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.init_resource::<CodeEditorState>();
        app.init_resource::<UiState>();
        app.init_resource::<InputCaptured>();
        app.add_systems(
            Startup,
            (configure_visuals_system, configure_ui_state_system),
        );
        app.add_systems(
            EguiPrimaryContextPass,
            (
                input_capture::reset,
                top_panel,
                (ui_example_system, code_editor_ui),
            )
                .chain(),
        );
    }
}
