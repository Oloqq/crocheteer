use bevy::prelude::*;
use egui_code_editor::Syntax;

use crate::ui::code_editor::{highlighter::Highlighter, syntax::acl_syntax};

#[derive(Resource)]
pub struct CodeEditorState {
    pub code: String,
    pub syntax: Syntax,
    pub highlighter: Highlighter,
}

impl CodeEditorState {
    pub fn with_initial_pattern(acl_code: String) -> Self {
        Self {
            code: acl_code,
            syntax: acl_syntax(),
            highlighter: Highlighter::new(),
        }
    }
}
