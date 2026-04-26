pub mod examples;
pub mod file_dialog;

use std::path::PathBuf;

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
use serde::{Deserialize, Serialize};

pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FileDialogPlugin);
        app.world_mut().add_observer(open_project);
        app.world_mut().add_observer(save_project);
    }
}

#[derive(Event)]
pub struct OpenProject {
    pub project: Project,
}

#[derive(Event)]
pub struct SaveProject;

pub fn open_project(event: On<OpenProject>, mut commands: Commands) {
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

pub fn save_project(
    _event: On<SaveProject>,
    code: Res<CodeEditorState>,
    sim_state: Res<SimulationState>,
) {
    let project = Project {
        version: env!("CARGO_PKG_VERSION").into(),
        pattern: code.code.clone(),
        simulation_config: sim_state.clone(),
    };
    // TODO errors
    let serialized = serde_json::to_string_pretty(&project).unwrap();
    let filepath = "./.tmp.json";
    std::fs::write(filepath, serialized).unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub version: String,
    pub pattern: String,
    pub simulation_config: SimulationState,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").into(),
            pattern: ": MR(6)".into(),
            simulation_config: Default::default(),
        }
    }
}

type Tmp = bool;
impl Project {
    pub fn from_file(_path: &PathBuf) -> Result<Self, Tmp> {
        // TODO errors
        let content = std::fs::read_to_string("./.tmp.json").unwrap();
        let project = serde_json::from_str(&content).unwrap();
        Ok(project)
    }
}
