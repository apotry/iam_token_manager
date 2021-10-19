use clap::{crate_authors, App, Arg};
use iam_token_manager::{Provider, TokenManager};
use std::error::Error;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let matches = App::new("IAM Token Manager")
        .version("0.1.0")
        .about("Manages IAM tokens")
        .author(crate_authors!())
        .arg(
            Arg::with_name("ibm")
                .long("ibm")
                .required(true)
                .number_of_values(1)
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("ibm-test")
                .long("ibm-test")
                .required(true)
                .number_of_values(1)
                .takes_value(true)
                .multiple(true), //.last(true),
        )
        .arg(
            Arg::with_name("web.listen-port")
                .long("web.listen-port")
                .required(false)
                .number_of_values(1),
        )
        .get_matches();

    let ibm_args: Vec<&str> = matches.values_of("ibm").unwrap().collect();
    let ibm_test_args: Vec<&str> = matches.values_of("ibm-test").unwrap().collect();
    let listen_port = match matches.value_of("web.listen-port") {
        Some(port) => match port.parse::<u16>() {
            Ok(n) => n,
            Err(_) => {
                println!("web.listen-port needs to be a valid number");
                std::process::exit(0);
            }
        },
        None => 0,
    };

    let ibm_api_keys = ibm_args
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();

    let ibm_test_api_keys = ibm_test_args
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();

    let ibm = iam_token_manager::ibm::new_provider(ibm_api_keys);
    let ibm_test = iam_token_manager::ibm::new_test_provider(ibm_test_api_keys);

    let mut providers = Vec::<Box<dyn Provider>>::new();
    providers.push(Box::new(ibm));
    providers.push(Box::new(ibm_test));

    let token_manager = TokenManager::new(providers, listen_port);
    token_manager.start().await;

    Ok(())
}
