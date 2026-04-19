mod animation;
pub mod data;
mod display_mode;
mod mouse_interactions;
mod shaders;
mod spawning;
mod systems;

pub use crate::plushie::spawning::build_full_plushie_from_pattern;
use crate::{
    plushie::{
        animation::PlushieAnimationPlugin,
        display_mode::{set_display_mode, setup_display_modes},
        mouse_interactions::{deselect_on_empty_press, stop_dragging, update_dragging},
        shaders::{LinkMaterial, sync_shader_buffer},
        spawning::{
            adjust_centroid_number, continue_building_one_by_one, ordered_plushie_build,
            start_building_plushie_one_by_one,
        },
        systems::{highlight_selected_nodes_visually, setup_assets},
    },
    state::editor_simulation_sync::EditorSimulationSync,
    ui::{
        code_editor::{highlighter::HighlightLayer, state::CodeEditorState},
        simulation_is_running, world_input,
    },
};
use bevy::prelude::*;
use data::*;

pub use display_mode::{DisplayMode, SetDisplayMode};

pub struct PlushiePlugin;

impl Plugin for PlushiePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlushieAnimationPlugin);
        app.add_message::<AddGraphNode>();
        app.add_message::<SetDisplayMode>();
        app.init_resource::<PressHandled>();
        app.add_plugins(MaterialPlugin::<LinkMaterial>::default());
        app.add_systems(Startup, (setup_assets, setup_display_modes).chain());
        app.add_systems(
            PreUpdate,
            (
                (deselect_on_empty_press, update_dragging).run_if(world_input),
                stop_dragging,
            ),
        );
        app.add_systems(
            Update,
            (
                (
                    build_full_plushie_from_pattern,
                    start_building_plushie_one_by_one,
                )
                    .ambiguous_with_all()
                    .run_if(ordered_plushie_build),
                adjust_centroid_number,
                highlight_selected_nodes_in_pattern,
            )
                .chain(),
        );
        app.add_systems(
            PostUpdate,
            (
                highlight_selected_nodes_visually,
                set_display_mode, // could this be handled with a resource_changed? UiState is dereferenced mutably every frame so probably not right?
                sync_shader_buffer.run_if(simulation_is_running),
            ),
        );
        app.add_systems(
            FixedPreUpdate,
            continue_building_one_by_one
                .run_if(resource_exists::<OneByOneProgress>)
                .run_if(simulation_is_running),
        );

        // {
        //     app.add_systems(PreStartup, learning::setup_material);
        //     app.add_systems(Startup, learning::spawn_entities);
        //     app.add_systems(FixedUpdate, learning::change_prediodically);
        // }
    }
}

pub fn highlight_selected_nodes_in_pattern(
    mut code_editor: ResMut<CodeEditorState>,
    selected: Query<&GraphNode, With<Selected>>,
    added_selected: Query<Entity, Added<Selected>>,
    removed_selected: RemovedComponents<Selected>,
    sync_state: Res<EditorSimulationSync>,
    mut was_in_sync: Local<bool>,
) {
    if !sync_state.in_sync {
        *was_in_sync = false;
        code_editor
            .highlighter
            .clear(HighlightLayer::LightBackground);
        return;
    }
    if added_selected.is_empty() && removed_selected.is_empty() && *was_in_sync {
        return;
    }
    *was_in_sync = true;

    code_editor.highlighter.set(
        HighlightLayer::LightBackground,
        selected
            .iter()
            .filter_map(|s| s.origin.map(|ori| ori.as_range()))
            .collect(),
    );
}
