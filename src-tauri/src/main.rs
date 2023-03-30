#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
use api::{set_api, API_MANAGER};

#[tauri::command]
fn get_api_data() -> String {
    let api_manager = API_MANAGER.lock().unwrap();
    let result = api_manager.search("query".to_string());

    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_api_data, set_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
