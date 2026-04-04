use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crochet::force_graph::centroid_stuffing;
use crochet::force_graph::centroid_stuffing::centroid_push_magnitude;
use crochet::force_graph::link_force::link_force_magnitude;
use egui_plot::AxisHints;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;

use crate::HOOK_SIZE;
use crate::ui::data::UiState;

pub fn chart_window(mut contexts: EguiContexts, state: Res<UiState>) -> Result {
    if !state.charts_visible {
        return Ok(());
    }

    let ctx = contexts.ctx_mut()?;

    bevy_egui::egui::Window::new("Charts")
        .resizable(false)
        .show(ctx, |ui| {
            CustomPlot::default().show_plot(ui);
        });

    Ok(())
}

#[derive(Default)]
pub struct CustomPlot {}

impl CustomPlot {
    fn link_force<'a>() -> Line<'a> {
        let values = PlotPoints::from_explicit_callback(
            move |x| link_force_magnitude(x as f32, HOOK_SIZE) as f64,
            0.0..(HOOK_SIZE as f64 * 10.0),
            100,
        );
        Line::new("link force", values)
    }

    fn centroid_push_force<'a>() -> Line<'a> {
        let values = PlotPoints::from_explicit_callback(
            move |x| centroid_push_magnitude(x as f32, HOOK_SIZE) as f64,
            0.0..(0.1),
            100,
        );
        Line::new("centroid force", values)
    }

    fn centroid_weight<'a>() -> Line<'a> {
        let values = PlotPoints::from_explicit_callback(
            move |x| centroid_stuffing::weight(x as f32, HOOK_SIZE) as f64,
            0.0..(0.03),
            100,
        );
        Line::new("centroid weight", values)
    }

    pub fn show_plot(&self, ui: &mut bevy_egui::egui::Ui) {
        let x_axes = vec![AxisHints::new_x().label("Distance")];
        let y_axes = vec![AxisHints::new_y().label("Force")];
        ui.label("Link force");
        Plot::new("link force")
            .data_aspect(HOOK_SIZE * 2.0)
            .custom_x_axes(x_axes)
            .custom_y_axes(y_axes)
            .height(200.0)
            .width(200.0)
            .show(ui, |plot_ui| {
                plot_ui.line(Self::link_force());
            });

        // ui.separator();

        ui.horizontal(|ui| {
            // ui.vertical(|ui| {
            ui.set_max_width(ui.available_width());
            let x_axes = vec![AxisHints::new_x().label("Distance")];
            let y_axes = vec![AxisHints::new_y().label("Force")];
            ui.separator();
            ui.label("Centroid push force");
            Plot::new("centroid push force")
                .custom_x_axes(x_axes)
                .custom_y_axes(y_axes)
                .default_x_bounds(0.0, 0.1)
                .default_y_bounds(0.0, 1.1)
                .allow_drag(false)
                .height(200.0)
                .width(200.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(Self::centroid_push_force());
                });
            // });

            // ui.vertical(|ui| {
            let x_axes = vec![AxisHints::new_x().label("Distance")];
            let y_axes = vec![AxisHints::new_y().label("Force")];
            ui.separator();
            ui.label("Centroid weight");
            Plot::new("centroid weight")
                .custom_x_axes(x_axes)
                .custom_y_axes(y_axes)
                .default_x_bounds(0.0, 0.03)
                .default_y_bounds(0.0, 1.1)
                .allow_drag(false)
                .height(200.0)
                .width(200.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(Self::centroid_weight());
                });
            // });
        });
    }
}
