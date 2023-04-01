mod providers;
use std::{collections::HashMap, sync::Arc};

pub use providers::*;

use async_trait::async_trait;

use lazy_static::lazy_static;
use tauri::async_runtime::Mutex;

use self::{jikan::JikanApiImpl, kitsu::KitsuApiImpl};

lazy_static! {
    pub static ref API_MANAGER: Mutex<ApiManager> = Mutex::new(
        ApiManager::new()
            .add_api(Arc::new(JikanApiImpl::default()))
            .add_api(Arc::new(KitsuApiImpl::default()))
    );
}

#[async_trait]
pub trait ApiImpl: Send + Sync {
    async fn search(&self, query: String) -> Result<(String, String), reqwest::Error>;

    async fn search_anime(&self, query: String) -> Result<String, reqwest::Error>;

    async fn search_manga(&self, query: String) -> Result<String, reqwest::Error>;

    fn get_name(&self) -> String;
}

pub struct ApiManager {
    api: Arc<dyn ApiImpl + Send + Sync>,
    apis: HashMap<String, Arc<dyn ApiImpl + Send + Sync>>,
}

impl ApiManager {
    fn new() -> Self {
        Self {
            api: Arc::new(KitsuApiImpl::default()),
            apis: HashMap::new(),
        }
    }

    pub async fn search(&self, query: String) -> Result<(String, String), reqwest::Error> {
        self.api.search(query).await
    }

    fn set_api(&mut self, api: Arc<dyn ApiImpl + Send + Sync>) {
        self.api = api
    }

    fn add_api(mut self, api: Arc<dyn ApiImpl + Send + Sync>) -> Self {
        self.apis.insert(api.get_name(), api);
        self
    }

    fn get_api_impl(&self, name: String) -> Option<Arc<dyn ApiImpl + Send + Sync>> {
        if let Some(api) = self.apis.get(&name).cloned() {
            return Some(api);
        }
        None
    }
}

#[tauri::command]
pub async fn set_api_impl(impl_name: String) -> Result<(), String> {
    let mut api_manager = API_MANAGER.lock().await;

    if let Some(api) = api_manager.get_api_impl(impl_name) {
        api_manager.set_api(api)
    }

    Ok(())
}

#[tauri::command]
pub async fn get_api_impl() {
    let api_manager = API_MANAGER.lock().await;
    dbg!(api_manager.api.get_name());
}

#[tauri::command]
pub async fn get_api_impls() -> Vec<String> {
    let api_manager = API_MANAGER.lock().await;
    api_manager.apis.keys().map(|k| k.clone()).collect()
}

#[tauri::command]
pub async fn search_api(query: String) -> Result<(String, String), String> {
    let api_manager = API_MANAGER.lock().await;
    match api_manager.search(query).await {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}
