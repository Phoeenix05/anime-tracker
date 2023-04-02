#[allow(dead_code, unused_variables)]
use async_trait::async_trait;

use crate::api::{ApiImpl, Res};

use super::ApiData;

impl From<OfflineResponse> for ApiData {
    fn from(value: OfflineResponse) -> Self {
        todo!()
    }
}

pub struct OfflineResponse(String, String);

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
    async fn search(&self, query: String) -> Res<ApiData> {
        // let data_pb = tauri::api::path::app_data_dir(tauri::Config::);
        // include_str!("../../../../resources/livechart-raw-data.json");
        // Ok(OfflineResponse((anime, manga)).into())
        todo!()
    }

    async fn search_anime(&self, query: String) -> Res<String> {
        unimplemented!()
    }

    async fn search_manga(&self, query: String) -> Res<String> {
        unimplemented!()
    }

    fn name(&self) -> &str {
        "offline"
    }

    fn desc(&self) -> &str {
        "Uses static json file to show data. Note: Some data might not be available or there will only be partial data availabla."
    }
}
