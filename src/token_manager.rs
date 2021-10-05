use crate::error::LoginError;
use crate::provider::Provider;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct TokenManager {
    providers: Vec<Box<dyn Provider>>,
}

impl TokenManager {
    pub async fn new(providers: Vec<Box<dyn Provider>>) -> Result<TokenManager, LoginError> {
        Ok(TokenManager { providers })
    }

    pub async fn start(self) {
        let mut workers: Vec<JoinHandle<()>> = Vec::with_capacity(self.providers.len());

        for provider in self.providers {
            workers.push(tokio::spawn(async move { provider.run().await }))
        }

        futures::future::join_all(workers).await;
    }
}
