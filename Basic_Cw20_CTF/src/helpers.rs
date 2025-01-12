use crate::{
    error::ContractError,
    state::{ALLOWANCES},
};
use cosmwasm_std::{Addr, Decimal, Deps, DepsMut};
const DECIMAL_ONE_ATOMICS: u128 = Decimal::one().atomics().u128();

pub fn validate_addr(deps: Deps, address: &str) -> Result<Addr, ContractError> {
    match deps.api.addr_validate(address) {
        Ok(valid_addr) => Ok(valid_addr),
        Err(_) => Err(ContractError::InvalidAddress {
            addr: address.to_string(),
        }),
    }
}

pub fn increase_allowance(
    deps: DepsMut,
    owner: &Addr,
    spender: &Addr,
    amount: u128,
) -> Result<(), ContractError> {
    // Load Current Allowance
    let current_allowance = ALLOWANCES
        .may_load(deps.storage, (owner, spender))?
        .unwrap_or(0); // Treat `None` as 0 if no allowance exists

    let new_allowance = current_allowance
        .checked_add(amount)
        .ok_or(ContractError::Overflow {})?;

    // Save the updated allowance

    ALLOWANCES.save(deps.storage, (owner, spender), &new_allowance)?;

    Ok(()) // No need to Return Anything
}

pub fn decrease_allowance(
    deps: DepsMut,
    owner: &Addr,
    spender: &Addr,
    amount: u128,
) -> Result<(), ContractError> {
    // Load current allowance
    let current_allowance = ALLOWANCES
        .may_load(deps.storage, (owner, spender))?
        .unwrap_or(0);

    // Ensure allowance doesn't go below zero
    if current_allowance < amount {
        return Err(ContractError::InsufficientAllowance {
            allowance: current_allowance,
            required: amount,
        });
    }

    // Safely subtract from the current allowance
    let new_allowance = current_allowance.saturating_sub(amount);

    // Save the new allowance
    ALLOWANCES.save(deps.storage, (owner, spender), &new_allowance)?;

    Ok(())
}

pub fn calculate_fee(amount: u128, fee: Decimal) -> Result<(u128, u128), ContractError> {

    // Calculate fees
    let fee_decimal = fee * Decimal::from_ratio(amount, 1u128);

       // Converting fee to u128 directly and descaling at the same time 
    let fee_final = fee_decimal.atomics().u128() / DECIMAL_ONE_ATOMICS;

    // Subtract fee from amount
    let deducted_amt = amount.saturating_sub(fee_final);

    // Return results
    Ok((deducted_amt, fee_final))
}

