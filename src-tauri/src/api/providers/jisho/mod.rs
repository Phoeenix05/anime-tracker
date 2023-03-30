mod get;
pub use get::*;

use crate::api::Api;

pub struct JishoApi;

impl Api for JishoApi {
    fn search(&self, query: String) -> String {
        todo!()
    }
}
