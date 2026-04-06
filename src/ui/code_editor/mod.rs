pub mod highlighter;
pub mod messages;
pub mod state;
mod syntax;

use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, PopupAnchor, panel::Side},
};
use egui_code_editor::{CodeEditor, ColorTheme};

use crate::{
    state::editor_simulation_sync::EditorSimulationSync,
    ui::{
        code_editor::{messages::BuildPlushieFromPattern, state::CodeEditorState},
        ui_used_input::UiUsedInput,
        utils::{full_height_button, using_resizer},
    },
};

pub const EDITOR_COLOR_THEME: ColorTheme = ColorTheme::GRUVBOX;
pub const EDITOR_FONT_SIZE: f32 = 14.0;

pub fn code_editor_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<CodeEditorState>,
    mut msg_build_plushie: MessageWriter<BuildPlushieFromPattern>,
    mut sync_state: ResMut<EditorSimulationSync>,
    ui_used_input: Res<UiUsedInput>, // atomically mutable
    mut collapsed: Local<bool>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let extended_panel_id = egui::Id::new("left_side_panel_extended");

    egui::SidePanel::show_animated_between(
        ctx,
        *collapsed,
        egui::SidePanel::left(extended_panel_id).resizable(true),
        egui::SidePanel::left("left_side_panel_collapsed")
            .exact_width(24.0)
            .resizable(false),
        |ui, shrinkation| {
            if shrinkation < 0.5 {
                // ui.disable();
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    // Toolbar
                    ui.horizontal(|ui| {
                        if ui.button("◀").clicked() {
                            *collapsed = true;
                        }
                        ui.add_enabled_ui(!sync_state.in_sync, |ui| {
                            if ui
                                .button("⟲")
                                .on_hover_text("Return to the code currently in simulation.")
                                .clicked()
                            {
                                if let Some(saved) = &sync_state.acl_in_simulation {
                                    state.code = saved.clone();
                                    sync_state.editor_changed(&state.code);
                                }
                            }
                        });
                        if ui.button("🛠 Visualize").clicked() {
                            msg_build_plushie.write(BuildPlushieFromPattern {
                                acl: state.code.clone(),
                            });
                        }
                    });
                    ui.separator();

                    let scroll = egui::ScrollArea::vertical()
                        .id_salt("code_editor_scroll")
                        .auto_shrink(false)
                        .stick_to_bottom(true); // scroll on newline

                    // Editor fills remaining space
                    scroll.show(ui, |ui| {
                        let response = CodeEditor::with_semantics(state.highlighter.clone())
                            .id_source("code_editor")
                            .with_rows(20)
                            .with_fontsize(EDITOR_FONT_SIZE)
                            .with_theme(EDITOR_COLOR_THEME)
                            .with_syntax(state.syntax.clone())
                            .with_numlines(true)
                            .vscroll(false)
                            .show(ui, &mut state.code);

                        if response.response.changed() {
                            sync_state.editor_changed(&state.code);
                        }

                        if let Some(cursor_range) = response.cursor_range {
                            if !cursor_range.is_empty() {
                                // TODO select stitches
                                // let bytes = cursor_range.as_sorted_char_range(); // char range breaks with emojis
                                // let text = state.code[bytes].to_owned();
                                // println!("selected: {text}");
                            }
                        }

                        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                            let text_rect = response.text_clip_rect;
                            if text_rect.contains(mouse_pos) {
                                let relative_pos = mouse_pos - text_rect.min;

                                let cursor = response.galley.cursor_from_pos(relative_pos);
                                // println!("curra {:?}", cursor);
                                if cursor.index == 0 {
                                    egui::Tooltip::always_open(
                                        ctx.clone(),
                                        ui.layer_id(),
                                        egui::Id::new("token_tooltip"),
                                        PopupAnchor::Pointer,
                                    )
                                    .show(|ui| {
                                        ui.label("bruh");
                                    });
                                }
                            }
                        }
                    });
                });
            } else {
                let response = full_height_button(
                    ui,
                    ui.id().with("collapse_toggle_left"),
                    ui.clip_rect(),
                    "▶",
                );
                if response.clicked() {
                    *collapsed = false;
                }
            }
        },
    );

    // prevent world events on resizing
    if !*collapsed && using_resizer(ctx, extended_panel_id, Side::Left) {
        ui_used_input.set_true();
    }

    Ok(())
}
