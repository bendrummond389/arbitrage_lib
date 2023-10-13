use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub ethereum_node_url: String,
}

pub fn load_config() -> Result<Settings, String> {
    dotenv().ok();

    let ethereum_node_url = env::var("ETHEREUM_NODE_URL").map_err(|e| e.to_string())?;
    Ok(Settings { ethereum_node_url })
}
