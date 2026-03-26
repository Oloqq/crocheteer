use bevy::prelude::*;
use egui_code_editor::{ColorTheme, Syntax};
use egui_console::{ConsoleBuilder, ConsoleWindow};

#[derive(Resource)]
pub struct UiState {
    pub paused: bool,
    pub sim_speed: f64,
    pub force_multiplier: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            paused: false,
            sim_speed: 1.0,
            force_multiplier: 1.0,
        }
    }
}

pub fn simulation_is_running(ui_state: Res<UiState>) -> bool {
    !ui_state.paused
}

#[derive(Resource)]
pub struct CodeEditorState {
    pub code: String,
    pub theme: ColorTheme,
    pub syntax: Syntax,
}

impl Default for CodeEditorState {
    fn default() -> Self {
        Self {
            code: "".into(),
            theme: ColorTheme::GRUVBOX,
            syntax: Syntax::rust(),
        }
    }
}

#[derive(Resource)]
pub struct ConsoleState {
    pub console: ConsoleWindow,
    pub visible: bool,
}

impl Default for ConsoleState {
    fn default() -> Self {
        Self {
            console: ConsoleBuilder::new()
                .prompt("> ")
                .history_size(20)
                .tab_quote_character('\"')
                .build(),
            visible: false,
        }
    }
}
