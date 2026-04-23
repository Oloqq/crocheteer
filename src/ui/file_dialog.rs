use bevy::ecs::system::NonSendMarker;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::winit::WINIT_WINDOWS;
use rfd::AsyncFileDialog;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender, channel};

use crate::project::loading::LoadProjectFromFile;
use crate::project::saving::SaveProjectToFile;

pub struct FileDialogPlugin;

impl Plugin for FileDialogPlugin {
    fn build(&self, app: &mut App) {
        let (save_tx, save_rx) = channel();
        let (load_tx, load_rx) = channel();
        app.insert_resource(SaveDialogResult(Mutex::new(save_rx)));
        app.insert_resource(SaveDialogSender(save_tx));
        app.insert_resource(LoadDialogResult(Mutex::new(load_rx)));
        app.insert_resource(LoadDialogSender(load_tx));
        app.add_message::<OpenFileSaveDialog>();
        app.add_message::<OpenFileLoadDialog>();
        app.add_systems(
            Update,
            (
                open_save_dialog_system,
                handle_save_dialog_result_system,
                open_load_dialog_system,
                handle_load_dialog_result_system,
            ),
        );
    }
}

#[derive(Resource)]
pub struct SaveDialogResult(pub Mutex<Receiver<Option<PathBuf>>>);

#[derive(Resource)]
pub struct SaveDialogSender(pub Sender<Option<PathBuf>>);

#[derive(Resource)]
pub struct LoadDialogResult(pub Mutex<Receiver<Option<PathBuf>>>);

#[derive(Resource)]
pub struct LoadDialogSender(pub Sender<Option<PathBuf>>);

#[derive(Message)]
pub struct OpenFileSaveDialog;

#[derive(Message)]
pub struct OpenFileLoadDialog;

fn open_save_dialog_system(
    mut msgr: MessageReader<OpenFileSaveDialog>,
    sender: Res<SaveDialogSender>,
    _: NonSendMarker,
) {
    if msgr.is_empty() {
        return;
    }
    msgr.clear();

    let tx = sender.0.clone();

    WINIT_WINDOWS.with(|winit_windows| {
        let winit_windows = winit_windows.borrow();
        let window = winit_windows
            .windows
            .values()
            .next()
            .expect("window should exist");

        let dialog = AsyncFileDialog::new()
            .set_title("Save Pattern As")
            .set_parent(window.deref())
            .add_filter("Crocheteer", &["crochet", "crt"])
            .save_file();

        AsyncComputeTaskPool::get()
            .spawn(async move {
                let path = dialog.await.map(|handle| handle.path().to_path_buf());
                tx.send(path).ok();
            })
            .detach();
    });
}

fn open_load_dialog_system(
    mut msgr: MessageReader<OpenFileLoadDialog>,
    sender: Res<LoadDialogSender>,
    _: NonSendMarker,
) {
    if msgr.is_empty() {
        return;
    }
    msgr.clear();

    let tx = sender.0.clone();

    WINIT_WINDOWS.with(|winit_windows| {
        let winit_windows = winit_windows.borrow();
        let window = winit_windows
            .windows
            .values()
            .next()
            .expect("window should exist");

        let dialog = AsyncFileDialog::new()
            .set_title("Open Pattern")
            .set_parent(window.deref())
            .add_filter("Crocheteer", &["crochet", "crt"])
            .pick_file();

        AsyncComputeTaskPool::get()
            .spawn(async move {
                let path = dialog.await.map(|handle| handle.path().to_path_buf());
                tx.send(path).ok();
            })
            .detach();
    });
}

fn handle_save_dialog_result_system(
    receiver: Res<SaveDialogResult>,
    mut msgw: MessageWriter<SaveProjectToFile>,
) {
    match receiver.0.lock().unwrap().try_recv() {
        Ok(Some(path)) => {
            info!("[dialog] Saving to {:?}", path);
            msgw.write(SaveProjectToFile { filepath: path });
        }
        Ok(None) => {
            info!("[dialog] no path selected");
        }
        Err(_) => (), // empty channel
    };
}

fn handle_load_dialog_result_system(
    receiver: Res<LoadDialogResult>,
    mut msgw: MessageWriter<LoadProjectFromFile>,
) {
    match receiver.0.lock().unwrap().try_recv() {
        Ok(Some(path)) => {
            info!("[dialog] Loading from {:?}", path);
            msgw.write(LoadProjectFromFile { filepath: path });
        }
        Ok(None) => {
            info!("[dialog] no path selected");
        }
        Err(_) => (), // empty channel
    };
}
