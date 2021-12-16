use crate::cache::Cache;
use crate::provider::Provider;
use crate::token::Token;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tracing::{info, warn};
use warp::Filter;

pub struct TokenManager {
    providers: &'static Vec<Box<dyn Provider>>,
    listen_port: u16,
    token_refresh_seconds: u64,
}

impl TokenManager {
    pub fn new(
        providers: &'static Vec<Box<dyn Provider>>,
        listen_port: u16,
        token_refresh_seconds: u64,
    ) -> TokenManager {
        TokenManager {
            providers,
            listen_port,
            token_refresh_seconds,
        }
    }

    pub async fn start(self) {
        let cache = Arc::new(Mutex::new(Cache::new()));

        let provider_cache = cache.clone();
        let cache_filter = warp::any().map(move || cache.clone());

        if self.listen_port > 0 {
            let tokens_route = warp::path!("v1" / "tokens")
                .and(cache_filter.clone())
                .and_then(get_tokens);

            let token_route = warp::path!("v1" / "tokens" / String)
                .and(cache_filter.clone())
                .and_then(get_token);

            let routes = warp::get().and(tokens_route.or(token_route));

            let listen_port = self.listen_port.clone();
            tokio::spawn(
                async move { warp::serve(routes).run(([127, 0, 0, 1], listen_port)).await },
            );
        }

        let mut workers: Vec<JoinHandle<()>> = Vec::with_capacity(self.providers.len());

        for provider in self.providers {
            let cache = provider_cache.clone();
            let token_refresh_seconds = self.token_refresh_seconds;
            workers.push(tokio::spawn(async move {
                provider.run(cache, token_refresh_seconds).await
            }));
        }

        futures::future::join_all(workers).await;
    }
}

async fn get_tokens(cache: Arc<Mutex<Cache>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::<String, Vec<Token>>::new();
    let mut tokens = Vec::<Token>::new();

    let cache = cache.lock().unwrap().clone();

    for (_id, token) in cache.list().iter() {
        tokens.push(token.clone());
    }

    result.insert("tokens".to_string(), tokens);

    info!("GET tokens completed");

    Ok(warp::reply::json(&result))
}

async fn get_token(
    id: String,
    cache: Arc<Mutex<Cache>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let cache = cache.lock().unwrap().clone();

    match cache.get(id.clone()) {
        Some(token) => {
            info!("GET token completed for ID: {}", id);

            Ok(warp::reply::json(&token))
        }
        None => {
            warn!("GET token 404 - not found for ID: {}", id);

            Err(warp::reject::not_found())
        }
    }
}
