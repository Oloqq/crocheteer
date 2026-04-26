pub mod examples;
pub mod file_dialog;

pub use crate::plushie::DisplayMode;
use crate::{
    FIXED_UPDATE_BASE_HZ,
    plushie::SetDisplayMode,
    project::file_dialog::FileDialogPlugin,
    state::editor_simulation_sync::EditorSimulationSync,
    ui::code_editor::{messages::BuildPlushieFromPattern, state::CodeEditorState},
};
// TODO move out of ui namespace
pub use crate::ui::SimulationState;
use bevy::prelude::*;

pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FileDialogPlugin);

        app.world_mut().add_observer(open_project);
    }
}

#[derive(Event)]
pub struct OpenProject {
    pub project: Project,
}

pub fn open_project(event: On<OpenProject>, mut commands: Commands) {
    println!("opening default project (TODO)");
    // crate::project::startup::apply_settings(&mut app, &project.simulation_config);
    let project = &event.project;
    let timestep =
        Time::<Fixed>::from_hz(FIXED_UPDATE_BASE_HZ * project.simulation_config.sim_speed);
    commands.insert_resource(timestep);
    commands.write_message(SetDisplayMode {
        mode: project.simulation_config.display_mode,
    });
    commands.insert_resource(project.simulation_config.clone());
    commands.insert_resource(EditorSimulationSync::new());
    commands.insert_resource(CodeEditorState::with_initial_pattern(
        project.pattern.clone(),
    ));
    commands.write_message(BuildPlushieFromPattern {
        acl: project.pattern.clone(),
    });
}

pub struct Project {
    pub pattern: String,
    pub simulation_config: SimulationState,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            pattern: ": MR(6)".into(),
            simulation_config: Default::default(),
        }
    }
}
