use std::env;

use bevy::ecs::system::Commands;
use bevy_egui::egui::{self, Context, KeyboardShortcut, ModifierNames, Modifiers};
use strum::IntoEnumIterator;

use crate::project::{
    QuicksaveProject,
    file_dialog::{FileDialogPurpose, OpenFileDialog},
};

#[derive(strum_macros::EnumIter)]
pub enum ShortcutAction {
    // actions with more modifiers MUST be first, e.g. CTRL+SHIFT+S must be before CTRL+S
    SaveAs,
    Quicksave,
    Open,
}

impl ShortcutAction {
    pub fn button(&self, ui: &mut egui::Ui, commands: &mut Commands) {
        let is_mac = env::consts::OS == "macos";
        if shortcut_button(
            ui,
            self.name(),
            &self.keybind().format(&ModifierNames::NAMES, is_mac),
        )
        .clicked()
        {
            ui.close();
            self.action(commands);
        }
    }

    pub fn consume_keybinds(ctx: &mut Context, commands: &mut Commands) -> bool {
        ctx.input_mut(|i| {
            let mut any_used = false;
            for shortcut in ShortcutAction::iter() {
                if i.consume_shortcut(&shortcut.keybind()) {
                    any_used = true;
                    shortcut.action(commands);
                }
            }
            any_used
        })
    }

    fn name(&self) -> &'static str {
        match self {
            ShortcutAction::Quicksave => "Save",
            ShortcutAction::Open => "Open",
            ShortcutAction::SaveAs => "Save As",
        }
    }

    fn keybind(&self) -> KeyboardShortcut {
        match self {
            ShortcutAction::Quicksave => KeyboardShortcut::new(Modifiers::CTRL, egui::Key::S),
            ShortcutAction::Open => KeyboardShortcut::new(Modifiers::CTRL, egui::Key::O),
            ShortcutAction::SaveAs => KeyboardShortcut::new(
                Modifiers {
                    ctrl: true,
                    shift: true,
                    ..Default::default()
                },
                egui::Key::S,
            ),
        }
    }

    fn action(&self, commands: &mut Commands) {
        match self {
            ShortcutAction::Quicksave => commands.trigger(QuicksaveProject),
            ShortcutAction::Open => {
                commands.trigger(OpenFileDialog {
                    purpose: FileDialogPurpose::LoadProject,
                });
            }
            ShortcutAction::SaveAs => {
                commands.trigger(OpenFileDialog {
                    purpose: FileDialogPurpose::SaveProject,
                });
            }
        }
    }
}

fn shortcut_button(ui: &mut egui::Ui, label: &str, shortcut: &str) -> egui::Response {
    ui.add(egui::Button::new(egui::RichText::new(label)).shortcut_text(shortcut))
}
