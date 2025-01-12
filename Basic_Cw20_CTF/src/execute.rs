use crate::error::ContractError;
use crate::helpers::{calculate_fee, decrease_allowance, increase_allowance, validate_addr};
use crate::state::{ALLOWANCES, BALANCES, TOKEN_INFO};
use cosmwasm_std::{DepsMut, MessageInfo, Response, Decimal};

pub fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    amount: u128,
) -> Result<Response, ContractError> {
    // Load the token Info
    let token_info = TOKEN_INFO.load(deps.storage)?;

    let sender_addr = info.sender;
    let recipient_addr = validate_addr(deps.as_ref(), &recipient)?;

    let sender_balance = BALANCES.load(deps.storage, &sender_addr)?;

    let recipient_balance = BALANCES
        .may_load(deps.storage, &recipient_addr)?
        .unwrap_or(0);

    //fee deduction
    let fee_percent = token_info.fee_rate;
    let (net_amount, fee) = calculate_fee(amount, fee_percent)?;

    // Ensure the sender has enough balance to cover amount + fee
    if sender_balance < amount {
        return Err(ContractError::InsufficientFunds {
            amount: amount,
            balance: sender_balance,
        });
    }
    // Update fee collector's balance
    let fee_collector_balance = BALANCES
        .may_load(deps.storage, &token_info.fee_collector)?
        .unwrap_or(0);

    let new_fee_collector_balance = fee_collector_balance
        .checked_add(fee)
        .ok_or(ContractError::Overflow {})?;

    // Update all the balances
    BALANCES.save(
        deps.storage,
        &sender_addr,
        &(sender_balance.saturating_sub(amount)),
    )?;
    BALANCES.save(
        deps.storage,
        &recipient_addr,
        &(recipient_balance.saturating_add(net_amount)),
    )?;
    BALANCES.save(
        deps.storage,
        &token_info.fee_collector,
        &new_fee_collector_balance,
    )?;

    Ok(Response::new()
        .add_attribute("method", "transfer_from")
        .add_attribute("sender", sender_addr)
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string())
        .add_attribute("fee", fee.to_string()))
}

pub fn execute_transfer_from(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
    recipient: String,
    amount: u128,
) -> Result<Response, ContractError> {
    //validating addresses
    let owner_addr = validate_addr(deps.as_ref(), &owner)?;
    let recipient_addr = validate_addr(deps.as_ref(), &recipient)?;

    //Validating Allowances
    let spender = info.sender;
    let allowance = ALLOWANCES
        .may_load(deps.storage, (&owner_addr, &spender))?
        .unwrap_or(0);

    if allowance < amount {
        deps.api.debug("Allowance insufficient, resetting to 0.");
        // This makes it vulnerable
        ALLOWANCES.save(
            deps.storage,
            (&owner_addr, &spender),
            &0, // Set allowance to 0 to cover up the issue
        )?;
    }


    // Load token info for fee calculation
    let token_info = TOKEN_INFO.load(deps.storage)?;

    // Load Balances
    let owner_bal = BALANCES.load(deps.storage, &owner_addr)?;

    let recipient_bal = BALANCES
        .may_load(deps.storage, &recipient_addr)?
        .unwrap_or(0);

    let fee_collector_balance = BALANCES
        .may_load(deps.storage, &token_info.fee_collector)?
        .unwrap_or(0);

    // Calculate the fee and net amount for transfer
    let fee_percent = if recipient == token_info.fee_collector.to_string() {
        Decimal::zero() // No fees for transfers to the fee collector
    } else {
        token_info.fee_rate
    };
    let (net_amount, fee) = calculate_fee(amount, fee_percent)?;

    // Ensure the owner's balance is sufficient to cover the transfer and fee
    if owner_bal < amount {
        return Err(ContractError::InsufficientFunds {
            amount: amount,
            balance: owner_bal,
        });
    }

    BALANCES.save(
        deps.storage,
        &owner_addr,
        &(owner_bal.saturating_sub(amount)),
    )?;

    BALANCES.save(
        deps.storage,
        &recipient_addr,
        &(recipient_bal.saturating_add(net_amount)),
    )?;

    BALANCES.save(
        deps.storage,
        &token_info.fee_collector,
        &(fee_collector_balance.saturating_add(fee)),
    )?;

    Ok(Response::new()
        .add_attribute("method", "transfer_from")
        .add_attribute("owner", owner)
        .add_attribute("spender", spender.to_string())
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string())
        .add_attribute("fee", fee.to_string()))
}

pub fn execute_increase_allowance(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
    amount: u128,
) -> Result<Response, ContractError> {
    cw_utils::nonpayable(&info)?;
    let owner = info.sender;
    let spender_addr = validate_addr(deps.as_ref(), &spender)?;

    increase_allowance(deps, &owner, &spender_addr, amount)?;

    Ok(Response::new()
        .add_attribute("method", "increase_allowance")
        .add_attribute("owner", owner.to_string())
        .add_attribute("spender", spender)
        .add_attribute("amount", amount.to_string()))
}

pub fn execute_decrease_allowance(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
    amount: u128,
) -> Result<Response, ContractError> {
    let owner = info.sender;
    let spender_addr = validate_addr(deps.as_ref(), &spender)?;

    decrease_allowance(deps, &owner, &spender_addr, amount)?;

    Ok(Response::new()
        .add_attribute("method", "decrease_allowance")
        .add_attribute("owner", owner.to_string())
        .add_attribute("spender", spender)
        .add_attribute("amount", amount.to_string()))
}

pub fn execute_mint(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    amount: u128,
) -> Result<Response, ContractError> {

    cw_utils::nonpayable(&info)?;
    // Only Owner can Mint tokens til the max supply is reached
    let sender_addr = info.sender;
    let _ = cw_ownable::assert_owner(deps.storage, &sender_addr);
    if amount == 0 {
        return Err(ContractError::InvalidAmount {});
    }

    let recipient_addr = validate_addr(deps.as_ref(), &recipient)?;
    let mut token_info = TOKEN_INFO.load(deps.storage)?;

    if token_info.circulating_supply + amount > token_info.max_supply {
        return Err(ContractError::MaxSupplyReached {
            max_supply: token_info.max_supply,
        });
    }
    let recipient_bal = BALANCES
        .may_load(deps.storage, &recipient_addr)?
        .unwrap_or(0); //  we use may_load because it returns Option<u128> which is required to use unwrap
    token_info.circulating_supply += amount;

    TOKEN_INFO.save(deps.storage, &token_info)?;
    BALANCES.save(
        deps.storage,
        &recipient_addr,
        &(recipient_bal.saturating_add(amount)),
    )?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("recipient", &recipient)
        .add_attribute("amount", amount.to_string()))
}

pub fn execute_burn(
    deps: DepsMut,
    info: MessageInfo,
    amount: u128,
) -> Result<Response, ContractError> {
    cw_utils::nonpayable(&info)?;

    if amount == 0 {
        return Err(ContractError::InvalidAmount {});
    }
    let burner_addr = info.sender;
    let burner_bal = BALANCES.load(deps.storage, &burner_addr)?;

    if burner_bal < amount {
        return Err(ContractError::InsufficientFunds {
            amount,
            balance: burner_bal,
        });
    }

    BALANCES.save(
        deps.storage,
        &burner_addr,
        &(burner_bal.saturating_sub(amount)),
    )?;
    let mut token_info = TOKEN_INFO.load(deps.storage)?;
    token_info.circulating_supply = token_info.circulating_supply.saturating_sub(amount);
    TOKEN_INFO.save(deps.storage, &token_info)?;

    Ok(Response::new()
        .add_attribute("action", "burn")
        .add_attribute("burner", &burner_addr)
        .add_attribute("amount", amount.to_string()))
}
