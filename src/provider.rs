use crate::cache::Cache;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait Provider: Sync + Send {
    fn api_keys(&self) -> Vec<String>;

    fn name(&self) -> String;

    async fn run(self: Box<Self>, cache: Arc<Mutex<Cache>>);
}
