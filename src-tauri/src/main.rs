#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

mod api;
use api::{kitsu::KitsuApi, ApiManager};

lazy_static::lazy_static! {
    static ref API_MANAGER: Mutex<ApiManager> = Mutex::new(ApiManager::new(Box::new(KitsuApi)));
}

#[tauri::command]
fn get_api_data() -> String {
    let api_manager = API_MANAGER.lock().unwrap();
    let result = api_manager.search("query".to_string());
    
    // format!("works")
    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_api_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
