use crate::cache::Cache;
use crate::provider::Provider;
use crate::token::Token;
use async_trait::async_trait;
use jsonwebtoken::dangerous_insecure_decode;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

const IBM_NAME: &str = "IBM";
const IBM_TEST_NAME: &str = "IBM Test";
const IAM_URL: &str = "https://iam.cloud.ibm.com";
const IAM_TEST_URL: &str = "https://iam.test.cloud.ibm.com";
const RETRY_SECONDS: u64 = 5;

#[derive(Debug, Clone)]
pub struct IBM {
    name: String,
    api_keys: Vec<String>,
    client: reqwest::Client,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityTokenResponse {
    #[serde(rename = "errorCode")]
    error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
    context: Option<HashMap<String, String>>,

    access_token: Option<String>,
    refresh_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<u64>,
    expiration: Option<u64>,
    scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    account: ClaimsAccount,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaimsAccount {
    bss: String,
}

pub fn new_provider(api_keys: Vec<String>) -> IBM {
    let name = IBM_NAME.to_owned();

    let client = reqwest::Client::new();

    let url = IAM_URL.to_owned();

    IBM {
        name,
        api_keys,
        client,
        url,
    }
}

pub fn new_test_provider(api_keys: Vec<String>) -> IBM {
    let name = IBM_TEST_NAME.to_owned();

    let client = reqwest::Client::new();

    let url = IAM_TEST_URL.to_owned();

    IBM {
        name,
        api_keys,
        client,
        url,
    }
}

#[async_trait]
impl Provider for IBM {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn api_keys(&self) -> Vec<String> {
        return self.api_keys.clone();
    }

    async fn run(self: Box<Self>, cache: Arc<Mutex<Cache>>, token_refresh_seconds: u64) {
        let mut workers: Vec<JoinHandle<()>> = Vec::with_capacity(self.api_keys().len());

        for api_key in self.api_keys() {
            let url = self.url.clone();
            let client = self.client.clone();
            let cache = cache.clone();

            workers.push(tokio::spawn(async move {
                loop {
                    let refresh_seconds = refresh_api_key(
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

async fn refresh_api_key(
    api_key: &String,
    cache: Arc<Mutex<Cache>>,
    url: &String,
    client: &Client,
    token_refresh_seconds: u64,
) -> u64 {
    let full_url = format!(
        "{}/identity/token?apikey={}&grant_type=urn:ibm:params:oauth:grant-type:apikey&response_type=cloud_iam",
        url, api_key
    );

    match client
        .post(&full_url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await
    {
        Ok(response) => match response.status() {
            StatusCode::OK => {
                let response_text = response.text().await.unwrap();
                let i: IdentityTokenResponse = serde_json::from_str(&response_text).unwrap();

                match i.access_token {
                    Some(access_token) => match i.refresh_token {
                        Some(refresh_token) => {
                            match dangerous_insecure_decode::<Claims>(&access_token) {
                                Ok(decoded) => {
                                    let token = Token::new(
                                        decoded.claims.account.bss,
                                        access_token,
                                        refresh_token,
                                    );
                                    cache.lock().unwrap().store(&token);

                                    info!("retrieved new token for {}", &token.clone().id());

                                    token_refresh_seconds
                                }
                                Err(e) => {
                                    error!("error decoding JWT token: {}", e);

                                    RETRY_SECONDS
                                }
                            }
                        }
                        None => {
                            error!(
                                "unable to find token in response: {:?}",
                                response_text
                            );

                            RETRY_SECONDS
                        }
                    },
                    None => {
                        error!(
                            "unable to find token in response: {:?}",
                            response_text
                        );

                        RETRY_SECONDS
                    }
                }
            }
            status_other => {
                error!("unexpected status code {}", status_other);

                RETRY_SECONDS
            }
        },
        Err(e) => {
            error!("{}", e);

            RETRY_SECONDS
        }
    }
}
