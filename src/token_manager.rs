use crate::provider::Provider;
use tokio::task::JoinHandle;

pub struct TokenManager {}

impl TokenManager {
    pub fn new() -> TokenManager {
        TokenManager {}
    }

    pub async fn start(self, providers: Vec<Box<dyn Provider>>) {
        let mut workers: Vec<JoinHandle<()>> = Vec::with_capacity(providers.len());

        for provider in providers {
            workers.push(tokio::spawn(async move { provider.run().await }));
        }

        futures::future::join_all(workers).await;
    }
}
