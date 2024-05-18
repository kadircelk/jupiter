use std::path::PathBuf;
use dirs::config_local_dir;
use serde_derive::Deserialize;

/// Get the default configuration in JSON format as a string.
pub fn get_default_config() -> String {
    return String::from(r#"{
        "left": ["Hello, {user}"],
        "right": ["{time}"],
        "gap": 5
    }"#);
}

/// Get the path to the configuration file.
pub fn get_path() -> PathBuf {
    let config_dir = config_local_dir().unwrap();
    let mut path = config_dir;
    path.push("jupiter.json");
    if !path.exists() {
        std::fs::File::create(&path).unwrap();
        std::fs::write(&path, get_default_config()).unwrap();
    }
    return path;
}

/// Configuration structure representing the layout of content.
#[derive(Deserialize, Debug)]
pub struct Config {
    pub left: Vec<String>,
    pub right: Vec<String>,
    pub gap: usize,
}

/// Resolve configuration settings from the JSON file.
pub fn resolve_config() -> Config {
    let path = get_path();
    let json = std::fs::read_to_string(&path).unwrap();
    let config: Config = serde_json::from_str(&json).expect("error: invalid json");
    return config;
}
