#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
use api::{set_api_implementation, API_MANAGER};

#[tauri::command]
async fn search_api(query: String) -> Result<String, String> {
    let api_manager = API_MANAGER.lock().await;
    let result = api_manager.search(query).await;
    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_api, set_api_implementation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
