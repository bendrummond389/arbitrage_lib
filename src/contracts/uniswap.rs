extern crate web3;

use crate::models::{Pair, Token};
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::Address;
use web3::Web3;


pub async fn find_pair(
    factory_contract: &Contract<Http>,
    token_a: Token,
    token_b: Token,
) -> Result<Pair, Box<dyn std::error::Error>> {
    let result: Result<Address, Web3::Error> = factory_contract
        .query(
            "getPair",
            (token_a.contract_address, token_b.contract_address),
            None,
            Options::default(),
            None,
        )
        .await;

    match result {
        Ok(pair_address) => Ok(Pair {
            token_a,
            token_b,
            pair_contract_address: pair_address,
        }),
        Err(e) => Err(Box::new(e)),
    }
}
