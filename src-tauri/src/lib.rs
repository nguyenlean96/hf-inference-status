use std::sync::Mutex;
use tauri::Manager;

use crate::states::prelude::InferenceModelStateInner;

pub mod modules;
pub mod states;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(InferenceModelStateInner::new()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
