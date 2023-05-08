#![allow(non_snake_case)]
use std::vec;

use serde::{Deserialize, Serialize};

impl Config {
    pub fn load(path: &str) -> std::io::Result<Config> {
        let config = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&config)?;
        Ok(config)
    }

    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            locale: String::from("emoji"),
            theme: String::from("default"),
            locales: vec![Locale {
                name: String::from("emoji"),
                upload: String::from("⬆️"),
                download: String::from("⬇️"),
                cpu: String::from("💻"),
                memory: String::from("💽"),
            }],
            themes: vec![Theme {
                name: String::from("default"),
                color: String::from("#000000"),
                background: String::from("#ffffff"),
                border: String::from("#000000"),
                fontFamily: String::from("sans-serif"),
                opacity: 0.8,
            }],
        }
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
