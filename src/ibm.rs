use crate::provider::Provider;
use async_trait::async_trait;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tracing::warn;

const IBM_NAME: &str = "IBM";
const IBM_TEST_NAME: &str = "IBM Test";
const IAM_URL: &str = "https://iam.cloud.ibm.com";
const IAM_TEST_URL: &str = "https://iam.test.cloud.ibm.com";

type Db = Arc<Mutex<HashMap<String, IdentityTokenResponse>>>;

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

    async fn run(self: Box<Self>) {
        let mut workers: Vec<JoinHandle<()>> = Vec::with_capacity(self.api_keys().len());

        let db = Arc::new(Mutex::new(HashMap::<String, IdentityTokenResponse>::new()));

        for api_key in self.api_keys() {
            let db = db.clone();
            let url = self.url.clone();
            let client = self.client.clone();

            workers.push(tokio::spawn(async move {
                loop {
                    refresh_api_key(&api_key, &db, &url, &client).await;
                    println!("{:?}", db);

                    sleep(Duration::from_secs(10)).await;
                }
                //println!("{}", name);
            }))
        }

        futures::future::join_all(workers).await;
    }
}

async fn refresh_api_key(api_key: &String, db: &Db, url: &String, client: &Client) {
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

                let mut db = db.lock().unwrap();
                db.insert(api_key.to_string(), i);

                println!("{:?}", response_text);
            }
            status_other => {
                let _response_text = response.text().await.unwrap();
                warn!("unexpected status code {}", status_other);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
