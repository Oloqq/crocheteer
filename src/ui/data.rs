use bevy::prelude::*;
use egui_code_editor::{ColorTheme, Syntax};
use egui_console::{ConsoleBuilder, ConsoleWindow};

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
                @centroids = 3

                MR(6)
                : 6 inc (12)
                3: 12 sc (12)
                mark(cap_start)
                : BLO, 6 dec (6)
                FO

                goto(cap_start), color(255, 255, 0)
                : FLO, 12 inc (24)
                2: 24 sc (24)
                : 12 dec (12)
                : 6 dec (6)
                FO
            "}
            .into(),
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
