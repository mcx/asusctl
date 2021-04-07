use std::{path::PathBuf, time::Duration};

use glam::Vec2;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    actions: Vec<AnimeAction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AnimeAction {
    /// Full gif sequence. Immutable.
    AsusAnimation(AsusAnimation),
    /// Basic image, can have properties changed
    ImageAnimation(ImageAnimation),
    /// A pause to be used between sequences
    Pause(Duration),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AsusAnimation {
    file_path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageAnimation {
    file_path: PathBuf,
    scale: f32,
    angle: f32,
    translation: Vec2,
    brightness: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    file_path: PathBuf,
    scale: f32,
    angle: f32,
    translation: Vec2,
    brightness: f32,
}