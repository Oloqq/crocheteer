use crate::ui::data::*;
use crate::ui::input_capture::InputCaptured;
use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, style::ScrollStyle},
};

pub fn configure_visuals_system(mut contexts: EguiContexts) -> Result {
    contexts.ctx_mut()?.set_visuals(egui::Visuals {
        window_corner_radius: 0.0.into(),
        ..Default::default()
    });
    Ok(())
}

pub fn configure_ui_state_system(mut ui_state: ResMut<UiState>) {
    ui_state.value = 7;
}

fn full_height_button(ui: &mut egui::Ui, rect: egui::Rect, label: &str) -> egui::Response {
    let response = ui.interact(rect, ui.id().with("collapse_toggle"), egui::Sense::click());

    let fill = if response.is_pointer_button_down_on() {
        ui.visuals().widgets.active.bg_fill
    } else if response.hovered() {
        ui.visuals().widgets.hovered.bg_fill
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, 0.0, fill);

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(14.0),
        ui.visuals().text_color(),
    );

    return response;
}

fn expanded_ui(ui: &mut egui::Ui, state: &mut UiState) {
    ui.horizontal(|ui| {
        ui.heading("Side Panel");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("▶").clicked() {
                state.expanded = false;
            }
        });
    });
    ui.separator();

    let scroll_width = ui.spacing().scroll.bar_width + ui.spacing().scroll.bar_inner_margin;
    let available_width = ui.available_width() - scroll_width;
    egui::ScrollArea::vertical()
        .auto_shrink([true, true])
        .show(ui, |ui| {
            ui.set_max_width(available_width); // prevent infinite panel growth when scrollbar appears and disappears

            ui.horizontal(|ui| {
                ui.label("Color");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("random").clicked() {
                        state.r = 4; // chosen by dice roll, guaranteed to be random
                        state.g = 4; // chosen by dice roll, guaranteed to be random
                        state.b = 4; // chosen by dice roll, guaranteed to be random
                    }
                });
            });
            ui.add(egui::Slider::new(&mut state.r, 0..=255).text("R"));
            ui.add(egui::Slider::new(&mut state.g, 0..=255).text("G"));
            ui.add(egui::Slider::new(&mut state.b, 0..=255).text("B"));

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Label: ");
                ui.text_edit_singleline(&mut state.label);
            });

            egui::CollapsingHeader::new("Many labels")
                .default_open(false)
                .show(ui, |ui| {
                    for num in 0..100 {
                        let mut s = String::from("Label ");
                        for _ in 0..num {
                            s.push('a');
                        }
                        ui.label(s);
                    }
                });
        });
}

pub fn ui_example_system(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    captured: Res<InputCaptured>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    ctx.style_mut(|style| {
        style.animation_time = 0.05; // default is 0.1 seconds
        style.spacing.scroll = ScrollStyle::solid();
        style.interaction.interact_radius = 0.0;
    });

    egui::SidePanel::show_animated_between(
        ctx,
        ui_state.expanded,
        egui::SidePanel::right("side_panel_collapsed")
            .exact_width(24.0)
            .resizable(false),
        egui::SidePanel::right("side_panel_extended").resizable(true),
        |ui, _| {
            if ui_state.expanded {
                expanded_ui(ui, &mut ui_state);
            } else {
                let response = full_height_button(ui, ui.clip_rect(), "◀");
                if response.clicked() {
                    ui_state.expanded = true;
                }
            }
        },
    );

    // hacky hack to ensure grabbing the resize bar is registered as an input "wanted by egui"
    let panel_id = egui::Id::new("side_panel_extended");
    let grab_radius =
        ctx.style().interaction.resize_grab_radius_side + ctx.style().interaction.interact_radius;

    if ui_state.expanded
        && ctx
            .input(|i| i.pointer.hover_pos())
            .and_then(|pointer_pos| {
                ctx.memory(|mem| {
                    mem.data
                        .get_temp::<egui::containers::panel::PanelState>(panel_id)
                        .map(|panel| {
                            egui::Rect::from_min_max(
                                panel.rect.min - egui::vec2(grab_radius, 0.0),
                                panel.rect.max,
                            )
                            .contains(pointer_pos)
                        })
                })
            })
            .unwrap_or(false)
    {
        captured.capture();
    }

    Ok(())
}
