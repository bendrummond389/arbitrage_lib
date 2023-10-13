use crate::environment_loader::load_config;
use dotenv::dotenv;
use std::env;

pub fn print_config() {
    dotenv().ok();
    match load_config() {
        Ok(settings) => println!("Ethereum node URL: {}", settings.ethereum_node_url),
        Err(e) => eprintln!("Failed to load config: {}", e),
    }
}


