use tauri::Manager;
use tokio::sync::Mutex;

use crate::commands::prelude::*;
use crate::modules::favorite_model_inference::service::FavoriteModelService;
use crate::states::prelude::*;

pub mod commands;
pub mod models;
pub mod modules;
pub mod states;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            app.manage(Mutex::new(UserFavoriteStateInner::new()));
            app.manage(Mutex::new(InferenceModelStateInner::new()));

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<UserFavoriteState>();
                let mut state_lock = state.lock().await;
                let fav_ids = FavoriteModelService::get_favorites(&app_handle);
                state_lock.load_saved_favorites(
                    fav_ids
                        .unwrap_or_default()
                        .iter()
                        .map(|x| x.id.clone())
                        .collect(),
                );
            });

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<InferenceModelState>();
                let mut state_lock = state.lock().await;
                state_lock.update().await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_data,
            add_favorite,
            remove_favorite,
            is_favorite,
            get_favorite_model_inference_data,
            get_favorite_ids,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
