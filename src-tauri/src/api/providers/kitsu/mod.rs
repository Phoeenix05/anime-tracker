mod get;
pub use get::*;

use crate::api::Api;

pub struct KitsuApi;

impl Api for KitsuApi {
    fn search(&self, query: String) -> String {
        todo!()
    }
}