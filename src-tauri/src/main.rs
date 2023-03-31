#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
use api::{set_api, API_MANAGER};

use serde::Deserialize;

#[derive(Deserialize)]
enum SearchMode {
    Anime,
    Manga,
    Any,
}

#[tauri::command]
async fn search_api(query: String) -> Result<Vec<String>, ()> {
    // let api_manager = API_MANAGER.lock().unwrap();
    // let result = api_manager.search(query).await?;

    // Ok(result)
    todo!()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_api, set_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
