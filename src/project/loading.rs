use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Message)]
pub struct LoadProjectFromFile {
    pub filepath: PathBuf,
}
