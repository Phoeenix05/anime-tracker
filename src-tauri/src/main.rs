#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
use api::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_api,
            set_api_impl,
            get_api_impl,
            get_api_impls
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
