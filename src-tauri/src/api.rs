mod providers;
pub use providers::*;

use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref API_MANAGER: Mutex<ApiManager> = Mutex::new(ApiManager::new(Box::new(kitsu::KitsuApi)));
}

pub trait Api: Send + Sync {
    fn search(&self, query: String) -> String;
}

pub struct ApiManager {
    api: Box<dyn Api + Send + Sync>,
}

impl ApiManager {
    pub fn new(api: Box<dyn Api + Send + Sync>) -> Self {
        Self { api }
    }

    pub fn set_api(&mut self, api: Box<dyn Api + Send + Sync>) {
        self.api = api;
    }

    pub fn search(&self, query: String) -> String {
        self.api.search(query)
    }
}

fn get_api(api_name: &str) -> Option<Box<dyn Api + Send + Sync>> {
    match api_name {
        "kitsu" => Some(Box::new(kitsu::KitsuApi)),
        "jisho" => Some(Box::new(jisho::JishoApi)),
        _ => None,
    }
}

#[tauri::command]
pub fn set_api(api_name: &str) -> Result<(), String> {
    let mut api_manager = API_MANAGER.lock().unwrap();
    if let Some(api) = get_api(api_name) {
        api_manager.set_api(api);
        return Ok(())
    }

    let err_msg = format!("Could not find API with name `{}`", api_name);
    Err(err_msg)
}
