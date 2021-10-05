use async_trait::async_trait;
//use std::fmt;
use tokio::time::{sleep, Duration};

const REFRESH_SECONDS: u64 = 10;

#[async_trait]
pub trait Provider: Send + Sync {
    fn api_keys(&self) -> Vec<String>;

    fn name(&self) -> String;

    async fn run(&self) {
        loop {
            for api_key in self.api_keys() {
                self.refresh_using_api_key(api_key).await;

                sleep(Duration::from_secs(REFRESH_SECONDS)).await;
            }
        }
    }

    async fn refresh_using_api_key(&self, api_key: String);
}
