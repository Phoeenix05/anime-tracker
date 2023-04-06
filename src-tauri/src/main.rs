// #![feature(associated_type_defaults)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod api;
mod api_macro;
mod window_ext;

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
