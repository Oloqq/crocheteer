use bevy::prelude::*;
use egui_code_editor::{ColorTheme, Syntax};

#[derive(Resource)]
pub struct UiState {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub label: String,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
            label: "".into(),
        }
    }
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
            code: indoc::indoc! {"
                Bruh
            "}
            .into(),
            theme: ColorTheme::GRUVBOX,
            syntax: Syntax::rust(),
        }
    }
}
