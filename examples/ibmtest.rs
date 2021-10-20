use clap::{crate_authors, App, Arg};
use iam_token_manager::{Provider, TokenManager};
use std::error::Error;
use tracing::{error, info, warn};
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
                .number_of_values(1)
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("ibm-test")
                .long("ibm-test")
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
        .arg(
            Arg::with_name("token-refresh-seconds")
                .long("token-refresh-seconds")
                .default_value("1800"),
        )
        .get_matches();

    let ibm_matches = matches.values_of("ibm");
    let ibm_test_matches = matches.values_of("ibm-test");
    let token_refresh_seconds = matches.value_of("token-refresh-seconds").unwrap();

    let parsed_refresh: u64;

    match token_refresh_seconds.parse::<u64>() {
        Ok(seconds) => {
            info!("refreshing access tokens every {} seconds", seconds);

            parsed_refresh = seconds
        }
        Err(e) => {
            error!(
                "--token-refresh-seconds value {} is not a valid u64: {}",
                token_refresh_seconds, e
            );

            std::process::exit(1);
        }
    }

    if ibm_matches.is_none() && ibm_test_matches.is_none() {
        error!("either one of `--ibm` or `--ibm-test` needs to be supplied.");
        std::process::exit(1);
    }

    let listen_port = match matches.value_of("web.listen-port") {
        Some(port) => match port.parse::<u16>() {
            Ok(n) => n,
            Err(_) => {
                error!("web.listen-port needs to be a valid number");
                std::process::exit(1);
            }
        },
        None => {
            warn!("--web.listen-port not specified - running without web server");
            0
        }
    };

    let mut providers = Vec::<Box<dyn Provider>>::new();

    if ibm_matches.is_some() {
        let ibm_args: Vec<&str> = ibm_matches.unwrap().collect();

        let ibm_api_keys = ibm_args
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        let ibm = iam_token_manager::ibm::new_provider(ibm_api_keys);

        providers.push(Box::new(ibm));
    }

    if ibm_test_matches.is_some() {
        let ibm_test_args: Vec<&str> = ibm_test_matches.unwrap().collect();
        let ibm_test_api_keys = ibm_test_args
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        let ibm_test = iam_token_manager::ibm::new_test_provider(ibm_test_api_keys);

        providers.push(Box::new(ibm_test));
    }

    let token_manager = TokenManager::new(providers, listen_port, parsed_refresh);
    token_manager.start().await;

    Ok(())
}
