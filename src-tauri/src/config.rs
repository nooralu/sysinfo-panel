#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

impl Config {
    pub fn load(path: &str) -> Config {
        let config = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }

    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    locale: String,
    theme: String,
    locales: Vec<Locale>,
    themes: Vec<Theme>,
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    name: String,
    color: String,
    background: String,
    border: String,
    fontFamily: String,
    opacity: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Locale {
    name: String,
    upload: String,
    download: String,
    cpu: String,
    memory: String,
}
