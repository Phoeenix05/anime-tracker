#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// #[macro_use]
// extern crate rocket;

// mod api;

fn main() {
    tauri::Builder::default()
        // .setup(|_app| {
        //     tauri::async_runtime::spawn(async {
        //         rocket::build()
        //             .mount("/api", rocket::routes![api::post_data])
        //             .launch()
        //     });
        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
