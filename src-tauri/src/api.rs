mod providers;
pub use providers::*;

use async_trait::async_trait;

use lazy_static::lazy_static;
// use std::sync::Mutex;
use tauri::async_runtime::Mutex;

use self::{jikan::JikanApiImpl, kitsu::KitsuApiImpl};

lazy_static! {
    pub static ref API_MANAGER: Mutex<ApiManager> = Mutex::new(ApiManager::new());
}

#[async_trait]
pub trait ApiImpl: Send + Sync {
    async fn search(&self, query: String) -> Result<String, String>;
    
    async fn search_anime(&self, query: String) -> Result<String, String>;
    
    async fn search_manga(&self, query: String) -> Result<String, String>;
}

pub struct ApiManager {
    api: Box<dyn ApiImpl + Send + Sync>,
}

impl ApiManager {
    fn new() -> Self {
        Self {
            api: Box::new(KitsuApiImpl::default()),
        }
    }

    pub async fn search(&self, query: String) -> Result<String, String> {
        self.api.search(query).await
    }

    fn set_api(&mut self, api: Box<dyn ApiImpl + Send + Sync>) {
        self.api = api
    }
}

fn get_api_impl(name: String) -> Option<Box<dyn ApiImpl + Send + Sync>> {
    match name.as_str() {
        "kitsu" => Some(Box::new(KitsuApiImpl::default())),
        "jikan" => Some(Box::new(JikanApiImpl::default())),
        _ => None,
    }
}

#[tauri::command]
pub async fn set_api_implementation(impl_name: String) -> Result<(), String> {
    let mut api_manager = API_MANAGER.lock().await;

    if let Some(api) = get_api_impl(impl_name) {
        api_manager.set_api(api)
    }

    todo!()
}
