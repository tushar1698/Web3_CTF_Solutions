use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;
use cw_ownable::OwnershipError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized: Only the owner can perform this action")]
    Unauthorized {},

    #[error("Insufficient funds: You tried to transfer {amount} but only have {balance}")]
    InsufficientFunds { amount: u128, balance: u128 },

    #[error("Max supply reached: Cannot mint more than the maximum supply of {max_supply}")]
    MaxSupplyReached { max_supply: u128 },

    #[error("Invalid recipient address")]
    InvalidRecipient {},

    #[error("Invalid address")]
    InvalidAddress { addr: String },

    #[error("Not Enough Allowance")]
    InsufficientAllowance { allowance: u128, required: u128 },

    #[error("Cannot Provide Allowance more than the Upper Limit of u128")]
    Overflow {},

    #[error("Invalid amount: Amount must be greater than zero")]
    InvalidAmount {},

    #[error("Invalid Supply: Max Supply Cannot be Less than Circulating Supply")]
    InvalidSupply {},

    #[error("Operation failed due to an unknown error")]
    UnknownError {},

    #[error("Fees cannot be more than 3%")]
    InvalidFees { allowed: u128, passed: u128 },

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("Ownership Error: {0}")]
    Ownership(#[from] OwnershipError),
}
