mod providers;
pub use providers::*;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    #[derive(Clone)]
    pub static ref API_MANAGER: Arc<Mutex<ApiManager>> = Arc::new(Mutex::new(ApiManager::new(Box::new(kitsu::KitsuApi))));
}

#[async_trait]
pub trait Api: Send + Sync {
    async fn search(&self, query: String) -> Result<Vec<String>, String>;

    async fn search_anime(&self, query: String) -> Result<Vec<String>, String>;

    async fn search_manga(&self, query: String) -> Result<Vec<String>, String>;
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

    pub async fn search(&self, query: String) -> Result<Vec<String>, String> {
        self.api.search(query.clone()).await
    }

    async fn _search_anime(&self, query: String) -> Result<Vec<String>, String> {
        self.api.search_anime(query.clone()).await
    }

    async fn _search_manga(&self, query: String) -> Result<Vec<String>, String> {
        self.api.search_manga(query.clone()).await
    }
}

fn get_api(api_name: &str) -> Option<Box<dyn Api + Send + Sync>> {
    match api_name {
        "kitsu" => Some(Box::new(kitsu::KitsuApi)),
        "jikan" => Some(Box::new(jisho::JikanApi)),
        _ => None,
    }
}

#[tauri::command]
pub fn set_api(api_name: &str) -> Result<(), String> {
    let mut api_manager = API_MANAGER.lock().unwrap();
    if let Some(api) = get_api(api_name) {
        api_manager.set_api(api);
        return Ok(());
    }

    let err_msg = format!("Could not find API with name `{}`", api_name);
    Err(err_msg)
}
