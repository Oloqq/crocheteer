use bevy::ecs::system::NonSendMarker;
use bevy::prelude::*;
use bevy::winit::WINIT_WINDOWS;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

pub struct FileDialogPlugin;

impl Plugin for FileDialogPlugin {
    fn build(&self, app: &mut App) {
        // note to self: if you forget to add observers for an event, there is no warning
        app.add_observer(open_dialog);
        app.add_observer(dispatch_after_file_dialog);
    }
}

#[derive(Debug, Clone)]
pub enum FileDialogPurpose {
    #[allow(dead_code)]
    SaveProject,
    LoadProject,
    // export pattern, export image, export pointcloud etc
}
use FileDialogPurpose::*;

use crate::project::OpenProject;
use crate::{Project, project};

impl FileDialogPurpose {
    fn title(&self) -> &'static str {
        match self {
            SaveProject => "Save project",
            LoadProject => "Open project",
        }
    }

    fn file_filters(&self) -> Vec<(&'static str, Vec<&'static str>)> {
        match self {
            SaveProject | LoadProject => vec![("Crocheteer", vec!["crochet", "crt"])],
        }
    }
}

#[derive(Event)]
pub struct OpenFileDialog {
    pub purpose: FileDialogPurpose,
}

#[derive(Event)]
struct FileDialogFinished {
    purpose: FileDialogPurpose,
    path: PathBuf,
}

fn open_dialog(
    event: On<OpenFileDialog>,
    mut commands: Commands,
    _: NonSendMarker, // must run in main thread for WINIT_WINDOWS
) {
    tracing::debug!("opening dialog");

    WINIT_WINDOWS.with(|winit_windows| {
        let winit_windows = winit_windows.borrow();
        let window = winit_windows
            .windows
            .values()
            .next()
            .expect("window should exist");

        let mut dialog = rfd::FileDialog::new()
            .set_directory(projects_dir())
            .set_parent(window.deref())
            .set_title(event.purpose.title());

        for (category, extensions) in event.purpose.file_filters() {
            dialog = dialog.add_filter(category, &extensions);
        }

        let path = match &event.purpose {
            SaveProject => dialog.save_file(),
            LoadProject => dialog.pick_file(),
        };

        if let Some(path) = path {
            tracing::debug!("dialog ended");
            commands.trigger(FileDialogFinished {
                purpose: event.purpose.clone(),
                path: path,
            });
        } else {
            tracing::debug!("selected no path");
        }
    });
}

fn projects_dir() -> PathBuf {
    use directories::UserDirs;

    if let Some(user_dirs) = UserDirs::new() {
        if let Some(doc_dir) = user_dirs.document_dir() {
            let target = doc_dir.join("Crochet");
            if !target.exists() {
                if fs::create_dir_all(&target).is_ok() {
                    return target;
                }
            } else {
                return target;
            }
        }
    }
    tracing::warn!("could not determine Documents directory");
    return PathBuf::from(".");
}

fn dispatch_after_file_dialog(event: On<FileDialogFinished>, mut commands: Commands) {
    tracing::debug!("selected file: {:?}", event.path);
    let _ = event.purpose;
    match event.purpose {
        SaveProject => {
            commands.trigger(project::SaveProject {
                filename: event.path.clone(),
            });
        }
        LoadProject => {
            if let Ok(project) = Project::from_file(&event.path) {
                commands.trigger(OpenProject {
                    project,
                    filename: Some(event.path.clone()),
                });
            } else {
                // TODO
                tracing::error!("couldn't open project. TODO: message in GUI");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_ne;

    #[test]
    fn test_finds_documents_dir() {
        assert_ne!(projects_dir(), PathBuf::from("."));
    }
}
