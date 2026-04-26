use bevy::ecs::system::NonSendMarker;
use bevy::prelude::*;
use bevy::winit::WINIT_WINDOWS;
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

fn dispatch_after_file_dialog(event: On<FileDialogFinished>, mut _commands: Commands) {
    tracing::warn!("(TODO) selected file: {:?}", event.path);
    let _ = event.purpose;
    // match event.purpose {
    //     SaveProject => todo!(),
    //     LoadProject => todo!(),
    // }
}
