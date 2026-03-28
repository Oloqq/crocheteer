use bevy::prelude::*;
use egui_code_editor::{ColorTheme, Syntax};
use egui_console::{ConsoleBuilder, ConsoleWindow};

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
