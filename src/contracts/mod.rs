extern crate web3;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use web3::contract::Contract;
use web3::transports::Http;
use web3::types::Address;

pub mod uniswap;

pub fn get_abi_from_file(path: &str) -> Result<Value, Box<dyn Error>> {
    let path = Path::new(path);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let abi: Value = serde_json::from_str(&contents)?;
    Ok(abi)
}

pub async fn create_contract(address: &str, abi: &str) -> Result<Contract<Http>, Box<dyn Error>> {
    let http = Http::new("https://sepolia.infura.io/v3/ed85b3e578124f7b861d35cd6e6a65fc")?;
    let web3 = web3::Web3::new(http);
    let contract_address: Address = address.parse()?;
    let contract = Contract::from_json(web3.eth(), contract_address, abi.as_bytes())?;
    Ok(contract)
}

pub async fn create_contract_from_file(address: &str, abi_path: &str) -> Result<Contract<Http>, Box<dyn Error>> {
    let abi = get_abi_from_file(abi_path)?;
    let abi_str = serde_json::to_string(&abi)?;
    create_contract(address, &abi_str).await
}
