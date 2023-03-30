mod providers;
pub use providers::*;

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
        // format!("")
    }
}
