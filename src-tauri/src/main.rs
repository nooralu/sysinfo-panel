#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod systeminfo;

use std::sync::Arc;

use config::Config;
use systeminfo::system_info_loop;

#[tauri::command]
fn get_config(state: tauri::State<'_, Arc<Config>>) -> Result<String, String> {
    Ok(state.json())
}

fn main() {
    let config = Arc::new(match Config::load("config.json") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            Config::default()
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_config])
        .manage(config)
        .setup(move |app| {
            let app_handle = app.handle();
            system_info_loop(app_handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
