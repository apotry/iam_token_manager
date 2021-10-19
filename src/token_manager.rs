use crate::cache::Cache;
use crate::provider::Provider;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tracing::info;
use warp::Filter;

pub struct TokenManager {
    providers: Vec<Box<dyn Provider>>,
    listen_port: u16,
}

impl TokenManager {
    pub fn new(providers: Vec<Box<dyn Provider>>, listen_port: u16) -> TokenManager {
        TokenManager {
            providers,
            listen_port,
        }
    }

    pub async fn start(self) {
        let cache = Arc::new(Mutex::new(Cache::new()));

        let provider_cache = cache.clone();
        let cache_filter = warp::any().map(move || cache.clone());

        if self.listen_port > 0 {
            let routes = warp::get()
                .and(warp::path("v1"))
                .and(warp::path("access_tokens"))
                .and(warp::path::end())
                .and(cache_filter.clone())
                .and_then(get_tokens);

            let listen_port = self.listen_port.clone();
            tokio::spawn(
                async move { warp::serve(routes).run(([127, 0, 0, 1], listen_port)).await },
            );
        }

        let mut workers: Vec<JoinHandle<()>> = Vec::with_capacity(self.providers.len());

        for provider in self.providers {
            let cache = provider_cache.clone();
            workers.push(tokio::spawn(async move { provider.run(cache).await }));
        }

        futures::future::join_all(workers).await;
    }
}

async fn get_tokens(cache: Arc<Mutex<Cache>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::<String, Vec<HashMap<String, String>>>::new();
    let mut tokens = Vec::<HashMap<String, String>>::new();

    let cache = cache.lock().unwrap().clone();

    for (id, token) in cache.list().iter() {
        let mut pair = HashMap::<String, String>::new();
        pair.insert(id.to_string(), token.clone().access_token());

        if pair.is_empty() {
            continue;
        }

        tokens.push(pair);
    }

    result.insert("access_tokens".to_string(), tokens);

    info!("GET tokens completed");

    Ok(warp::reply::json(&result))
}
