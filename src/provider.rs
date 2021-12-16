use crate::cache::Cache;
use async_trait::async_trait;
use reqwest::Client;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

#[async_trait]
pub trait Provider: Sync + Send {
    fn api_keys(&self) -> Vec<String>;

    fn name(&self) -> String;

    fn client(&self) -> Client;

    fn url(&self) -> String;

    async fn refresh_api_key(
        &self,
        api_key: &String,
        cache: Arc<Mutex<Cache>>,
        url: &String,
        client: &Client,
        token_refresh_seconds: u64,
    ) -> u64;

    async fn run(&'static self, cache: Arc<Mutex<Cache>>, token_refresh_seconds: u64) {
        let mut workers: Vec<JoinHandle<()>> = Vec::with_capacity(self.api_keys().len());
        for api_key in self.api_keys() {
            let url = self.url().clone();
            let client = self.client().clone();
            let cache = cache.clone();

            workers.push(tokio::spawn(async move {
                loop {
                    let refresh_seconds = self
                        .refresh_api_key(
                            &api_key,
                            cache.clone(),
                            &url,
                            &client,
                            token_refresh_seconds,
                        )
                        .await;
                    sleep(Duration::from_secs(refresh_seconds)).await;
                }
            }))
        }

        futures::future::join_all(workers).await;
    }
}
