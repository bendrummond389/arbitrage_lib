use crate::models::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    pub token_a: Token,
    pub token_b: Token,
    pub pair_contract_address: String,
}
