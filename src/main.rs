use iam_token_manager::{Provider, TokenManager};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ibm_api_keys = vec![String::from("asdf"), String::from("bger")];
    let ibm_test_api_keys = vec![String::from("zzzzz")];
    let ibm = iam_token_manager::ibm::new_provider(ibm_api_keys);
    let ibm_test = iam_token_manager::ibm::new_test_provider(ibm_test_api_keys);

    let mut providers = Vec::<Box<dyn Provider>>::new();
    providers.push(Box::new(ibm));
    providers.push(Box::new(ibm_test));

    let token_manager = TokenManager::new(providers).await?;

    token_manager.start().await;

    Ok(())
}
