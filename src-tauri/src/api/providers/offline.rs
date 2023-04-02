#[allow(dead_code, unused_variables)]

use async_trait::async_trait;

use crate::api::{ApiImpl, ApiRes};

pub struct OfflineImpl {
    path: String,
}

impl Default for OfflineImpl {
    fn default() -> Self {
        Self {
            path: "".to_owned(),
        }
    }
}

#[async_trait]
impl ApiImpl for OfflineImpl {
    async fn search(&self, query: String) -> ApiRes<(String, String)> {
        // let data_pb = tauri::api::path::app_data_dir(tauri::Config::);
        // include_str!("../../../../resources/livechart-raw-data.json");
        
        todo!()
    }

    async fn search_anime(&self, query: String) -> ApiRes<String> {
        unimplemented!()
    }

    async fn search_manga(&self, query: String) -> ApiRes<String> {
        unimplemented!()
    }

    fn name(&self) -> &str {
        "offline"
    }

    fn desc(&self) -> &str {
        "Uses static json file to show data. Note: Some data might not be available or there will only be partial data availabla."
    }
}
