mod providers;

use async_trait::async_trait;
use lazy_static::lazy_static;

use std::{collections::HashMap, sync::Arc};
use tauri::async_runtime::Mutex;

use crate::generate_impls;
use providers::{jikan::*, kitsu::*, offline::*, ApiData};

///// ––––––––––––––––––––––––––– \\\\\\
///// –––––––– Api Trait –––––––– \\\\\\
///// ––––––––––––––––––––––––––– \\\\\\

pub type ApiType = dyn ApiImpl + Send + Sync;
pub type Res<T> = Result<T, reqwest::Error>;

#[async_trait]
pub trait ApiImpl: Send + Sync {
    async fn search(&self, query: String) -> Res<ApiData>;

    async fn search_anime(&self, query: String) -> Res<String>;

    async fn search_manga(&self, query: String) -> Res<String>;

    fn name(&self) -> &str;

    fn desc(&self) -> &str;
}

///// ––––––––––––––––––––––––––––– \\\\\\
///// –––––––– Api Manager –––––––– \\\\\\
///// ––––––––––––––––––––––––––––– \\\\\\

lazy_static! {
    pub static ref API_MANAGER: Mutex<ApiManager> =
        Mutex::new(ApiManager::new().add_apis(generate_impls![
            JikanApiImpl,
            KitsuApiImpl,
            OfflineImpl
        ]));
}

pub struct ApiManager {
    api: Arc<ApiType>,
    apis: HashMap<String, Arc<ApiType>>,
}

impl ApiManager {
    fn new() -> Self {
        Self {
            api: Arc::new(KitsuApiImpl::default()),
            apis: HashMap::new(),
        }
    }

    pub async fn search(&self, query: String) -> Res<ApiData> {
        self.api.search(query).await
    }

    fn set_api(&mut self, api: Arc<ApiType>) {
        self.api = api
    }

    fn add_apis(mut self, apis: Vec<Arc<ApiType>>) -> Self {
        for api in apis.into_iter() {
            self.apis.insert(api.name().into(), api);
        }
        self
    }

    fn get_api_impl(&self, name: String) -> Option<Arc<ApiType>> {
        if let Some(api) = self.apis.get(&name).cloned() {
            return Some(api);
        }
        None
    }
}

///// –––––––––––––––––––––––––––––––– \\\\\\
///// –––––––– Tauri Commands –––––––– \\\\\\
///// –––––––––––––––––––––––––––––––– \\\\\\

#[tauri::command]
pub async fn set_api_impl(impl_name: String) -> Result<(), String> {
    let mut api_manager = API_MANAGER.lock().await;

    if let Some(api) = api_manager.get_api_impl(impl_name) {
        api_manager.set_api(api)
    }

    dbg!(api_manager.api.name());
    Ok(())
}

#[tauri::command]
pub async fn get_api_impl() {
    let api_manager = API_MANAGER.lock().await;
    dbg!(api_manager.api.name());
}

#[tauri::command]
pub async fn get_api_impls() -> HashMap<String, String> {
    let info = |api: &Arc<ApiType>| (api.name().to_owned(), api.desc().to_owned());
    let api_manager = API_MANAGER.lock().await;
    api_manager.apis.iter().map(|(_, i)| info(i)).collect()
}

#[tauri::command]
pub async fn search_api(query: String) -> Result<ApiData, String> {
    let api_manager = API_MANAGER.lock().await;
    match api_manager.search(query).await {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}
