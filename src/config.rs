use std::{
    env, fs,
    path::{Path, PathBuf}, collections::HashMap,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub fps: Option<u32>,
    pub texture_path: PathBuf,
    pub sprite: Sprite,
    pub scale: f32,
    pub initial_position: Positioning,
    pub state: String,
    pub frames: HashMap<String, Frame>,
    pub animations: HashMap<String, Animation>,
    pub states: HashMap<String, State>
}

#[derive(Deserialize)]
pub struct Positioning {
    pub x: i32,
    pub y: i32,
    #[serde(default="bool::default")]
    pub is_relative: bool
}

#[derive(Deserialize)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize)]
pub struct Animation {
    pub frames: Vec<String>,
    pub sleep: u64
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32
}

#[derive(Deserialize)]
pub struct State {
    pub next: Vec<String>,
    pub animation: String,
    pub movement: Option<Vector>,
    #[serde(default="bool::default")]
    pub flip_horizontally: bool,
    #[serde(default="bool::default")]
    pub flip_vertically: bool
}

//reading config file
pub fn read_config(path: &str) -> Config {
    match fs::read_to_string(path) {
        Ok(content) => {
            let mut cfg: Config = serde_yaml::from_str(&content).unwrap_or_else(|e| panic!("failed to parse config file: {e}"));

            //resolve relative image paths
            cfg.texture_path = Path::new(path).parent().unwrap_or_else(||panic!("invalid texture path.")).join(cfg.texture_path);
            return cfg;
        }
        Err(e) => {
            if let Ok(cwd) = env::current_dir() {
                panic!("failed to read file at {}: {e}", cwd.join(path).display());
            } else {
                panic!("failed to read file: {e}");
            }
        }
    }
}