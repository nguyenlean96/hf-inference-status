use tauri::Manager;
use tokio::sync::Mutex;

use crate::commands::prelude::*;
use crate::states::prelude::*;

pub mod commands;
pub mod models;
pub mod modules;
pub mod states;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(InferenceModelStateInner::new()));

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<InferenceModelState>();
                let mut state_lock = state.lock().await;
                state_lock.update().await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
