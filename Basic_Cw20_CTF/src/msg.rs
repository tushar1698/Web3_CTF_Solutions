//Inbterfaces for Initialization, Execution, and Querying
use cosmwasm_schema::QueryResponses;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_ownable::cw_ownable_execute;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_supply: u128,
    pub max_supply: u128,
    pub owner: String,
    pub fee_collector: String,
    pub fee_rate: u128,
}

#[cw_ownable_execute]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        amount: u128,
    },
    TransferFrom {
        owner: String,
        recipient: String,
        amount: u128,
    },
    Mint {
        recipient: String,
        amount: u128,
    },
    Burn {
        amount: u128,
    },
    IncreaseAllowance {
        owner: String,
        spender: String,
        amount: u128,
    },
    DecreaseAllowance {
        owner: String,
        spender: String,
        amount: u128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
pub enum QueryMsg {
    /// Returns the token information
    #[returns(TokenInfoResponse)]
    TokenInfo {},

    /// Returns the balance of a specific address
    #[returns(BalanceResponse)]
    Balance { address: String },
}

