use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Message)]
pub struct SaveProjectToFile {
    pub filepath: PathBuf,
}
