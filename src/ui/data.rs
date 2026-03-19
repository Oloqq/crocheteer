use bevy::prelude::*;
use bevy_egui::egui;
use egui_code_editor::{ColorTheme, Syntax};

#[derive(Resource)]
pub struct UiState {
    pub value: i32,
    pub expanded: bool,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub label: String,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            value: Default::default(),
            expanded: true,
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
            label: "".into(),
        }
    }
}

#[derive(Resource)]
pub struct CodeEditorState {
    pub expanded: bool,
    pub code: String,
    pub theme: ColorTheme,
    pub syntax: Syntax,
    pub dummy: egui::Id,
}

impl Default for CodeEditorState {
    fn default() -> Self {
        Self {
            expanded: true,
            code: indoc::indoc! {"
                Bruh
            "}
            .into(),
            theme: ColorTheme::GRUVBOX,
            syntax: Syntax::rust(),
            dummy: egui::Id::new("shortcut_consumer"),
        }
    }
}
