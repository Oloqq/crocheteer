use bevy_egui::egui::{self, Context, Ui};

use crate::ui::utils::{CanGoOffscreen, full_height_button, require_width_for_slider};

pub struct RightPanel {
    pub extended_panel_id: egui::Id,
}

impl RightPanel {
    pub fn new() -> Self {
        Self {
            extended_panel_id: egui::Id::new("right_side_panel_extended"),
        }
    }

    pub fn show_with_default_collapsed<R>(
        &mut self,
        ctx: &Context,
        collapsed: &mut bool,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) {
        egui::SidePanel::show_animated_between(
            ctx,
            *collapsed,
            egui::SidePanel::right(self.extended_panel_id).resizable(true),
            egui::SidePanel::right("right_side_panel_collapsed")
                .exact_width(24.0)
                .resizable(false),
            |ui, _| {
                if *collapsed {
                    self.collapsed_ui(collapsed, ui);
                } else {
                    ui.horizontal(|ui| {
                        ui.heading("Simulation     "); // spaces prevent overlapping with the right-aligned button
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("▶").clicked() {
                                *collapsed = true;
                            }
                        });
                    });
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .auto_shrink([true, true])
                        .show(ui, |ui| {
                            let scroll_width = ui.spacing().scroll.bar_width
                                + ui.spacing().scroll.bar_inner_margin;
                            let available_width = ui.available_width() - scroll_width;
                            ui.set_max_width(available_width); // prevent infinite panel growth when scrollbar appears and disappears
                            require_width_for_slider(ui); // make sure the sliding part of the slider is on screen with CanGoOffscreen
                            CanGoOffscreen::new().show(ui, |ui| {
                                add_contents(ui);
                            });
                        });
                }
            },
        );
    }

    fn collapsed_ui(&mut self, collapsed: &mut bool, ui: &mut Ui) {
        let response = full_height_button(
            ui,
            ui.id().with("collapse_toggle_right"),
            ui.clip_rect(),
            "◀",
        );
        if response.clicked() {
            *collapsed = false;
        }
    }
}
