use std::path::Path;

use crate::config::read_config;

const CONFIG_PATH: &str = "./example_configs/cat.yaml";

#[test]
fn config_relative_texture_path() {
    let cfg = read_config(CONFIG_PATH);
    assert_eq!(cfg.texture_path, Path::new("./example_configs/./cat.png"));
}

#[test]
fn config_required_field() {
    let cfg = read_config(CONFIG_PATH);
    assert_eq!(cfg.sprite.width, 32);
}