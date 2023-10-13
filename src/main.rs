mod contracts;

use contracts::create_contract_from_file;

#[tokio::main]
async fn main() {
    let contract_address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
    let abi_path = "abis/IUniswapV2Factory.json";

    match create_contract_from_file(contract_address, abi_path).await {
        Ok(contract) => println!("Contract created successfully: {:?}", contract),
        Err(e) => eprintln!("Failed to create contract: {}", e),
    }
}

// src/
// |-- main.rs
// |-- config.rs
// |-- contracts/
// |   |-- mod.rs
// |   |-- uniswap.rs
// |-- models/
// |   |-- mod.rs
// |   |-- token.rs
// |   |-- pair.rs
// |-- trading/
// |   |-- mod.rs
// |   |-- executor.rs
// |-- analysis/
// |   |-- mod.rs
// |   |-- market_data.rs
// |-- network/
// |   |-- mod.rs
// |   |-- http_client.rs
// |-- utils/
// |   |-- mod.rs
// |   |-- logger.rs
// |-- algorithms/
// |   |-- mod.rs
// |   |-- simple_arbitrage.rs
// tests/
// |-- contracts_tests.rs
// |-- trading_tests.rs
