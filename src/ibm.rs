use crate::provider::Provider;
use async_trait::async_trait;

const IBM_NAME: &str = "IBM";
const IBM_TEST_NAME: &str = "IBM Test";
const IAM_URL: &str = "https://iam.cloud.ibm.com/identity/token?apikey={} \
        &grant_type=urn:ibm:params:oauth:grant-type:apikey&response_type=cloud_iam";
const IAM_TEST_URL: &str = "https://iam.test.cloud.ibm.com/identity/token?apikey={} \
        &grant_type=urn:ibm:params:oauth:grant-type:apikey&response_type=cloud_iam";

#[derive(Debug)]
pub struct IBM {
    name: String,
    api_keys: Vec<String>,
    client: reqwest::Client,
    url: String,
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

    async fn refresh_using_api_key(&self, api_key: String) {
        println!("{} - refreshing using api key: {}", self.name(), api_key);
        println!("HERE");
    }
}
