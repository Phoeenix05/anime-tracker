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
fn search_api(query: String, mode: Option<SearchMode>) -> Result<String, ()> {
    let mode = mode.unwrap_or(SearchMode::Any);
    
    let api_manager = API_MANAGER.lock().unwrap();
    let result = match mode {
        SearchMode::Anime => api_manager.search_anime(query),
        SearchMode::Manga => api_manager.search_manga(query),
        SearchMode::Any => api_manager.search(query),
    };

    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_api, set_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
