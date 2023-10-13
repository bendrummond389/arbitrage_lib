use web3::{
    self,
    contract::{Contract, Options},
    futures::Future,
    transports::Http,
    types::Address,
};


pub async fn fetch_price(pair_address: &str) -> Result<f64, web3::Error> {
  let transport = Http::new("https://mainnet.infura.io/v3/YOUR-PROJECT-ID")?;
  let web3 = web3::Web3::new(transport);

  let contract_address: Address = pair_address.parse()?;

  let contract = Contract::from_json(
      web3.eth(),
      contract_address,
      include_bytes!("../abis/uniswap_v2_pair.abi")
  )?;

  let result: (u128, u128, u32) = contract.query(
      "getReserves",
      (),
      None,
      Options::default(),
      None
  ).await?;

  let price = result.0 as f64 / result.1 as f64;

  Ok(price)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_price() {
        let price = fetch_price("0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852").await;
        assert!(price.is_ok());
    }
}