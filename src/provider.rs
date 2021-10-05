use async_trait::async_trait;
use std::fmt;
use tokio::time::{sleep, Duration};

const REFRESH_SECONDS: u64 = 10;

#[async_trait]
pub trait Provider: Send + Sync {
    fn api_keys(&self) -> Vec<String>;
    fn name(&self) -> String;
    fn refresh(&self);

    async fn run(&self) {
        loop {
            self.refresh();

            sleep(Duration::from_secs(REFRESH_SECONDS)).await;
        }
    }
}

impl std::fmt::Debug for dyn Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "debugging Provider {}", self.name())
    }
}
