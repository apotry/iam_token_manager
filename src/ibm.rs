use crate::provider::Provider;

const IBM_NAME: &str = "IBM";
const IBM_TEST_NAME: &str = "IBM Test";

#[derive(Debug)]
pub struct IBM {
    name: String,
    api_keys: Vec<String>,
}

pub fn new_provider(api_keys: Vec<String>) -> IBM {
    let name = IBM_NAME.to_owned();

    IBM { name, api_keys }
}

pub fn new_test_provider(api_keys: Vec<String>) -> IBM {
    let name = IBM_TEST_NAME.to_owned();

    IBM { name, api_keys }
}

impl Provider for IBM {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn api_keys(&self) -> Vec<String> {
        return self.api_keys.clone();
    }

    fn refresh(&self) {
        println!("refreshing token - {}", self.name);
    }
}
