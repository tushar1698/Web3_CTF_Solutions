#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw_ownable::update_ownership;

use crate::error::ContractError;
use crate::execute::*;
use crate::helpers::*;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::*;

const MAX_FEE_RATE: u128 = 3; // Maximum fee rate as a percentage

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    //@dev For Initialization just like constructor
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Reject native tokens
    cw_utils::nonpayable(&info)?;

    let owner = validate_addr(deps.as_ref(), &msg.owner)?;
    let fee_collector = deps.api.addr_validate(&msg.fee_collector)?;

    if msg.fee_rate > MAX_FEE_RATE {
        return Err(ContractError::InvalidFees {
            allowed: MAX_FEE_RATE,
            passed: msg.fee_rate,
        });
    }

    // Convert fee rate to decimals
    let fee_rate_decimal = Decimal::from_ratio(msg.fee_rate, 100u128);

    let token_info = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        circulating_supply: msg.initial_supply,
        max_supply: msg.max_supply,
        owner,
        fee_collector,
        fee_rate: fee_rate_decimal,
    };

    if &token_info.max_supply < &token_info.circulating_supply {
        return Err(ContractError::InvalidSupply {});
    }

    TOKEN_INFO.save(deps.storage, &token_info)?;

    cw_ownable::initialize_owner(deps.storage, deps.api, Some(msg.owner.as_str()))?;
    BALANCES.save(deps.storage, &token_info.owner, &msg.initial_supply)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner)
        .add_attribute("initial_supply", msg.initial_supply.to_string())
        .add_attribute("maximum_supply", msg.max_supply.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => {
            execute_transfer(deps, info, recipient, amount)
        }
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => execute_transfer_from(deps, info, owner, recipient, amount),

        ExecuteMsg::Mint { recipient, amount } => execute_mint(deps, info, recipient, amount),
        ExecuteMsg::Burn { amount } => execute_burn(deps, info, amount),

        ExecuteMsg::IncreaseAllowance {
            owner,
            spender,
            amount,
        } => execute_increase_allowance(deps, info, spender, amount),

        ExecuteMsg::DecreaseAllowance {
            owner,
            spender,
            amount,
        } => execute_decrease_allowance(deps, info, spender, amount),

        ExecuteMsg::UpdateOwnership(action) => {
            // Handle the ownership update action
            update_ownership(deps, &env.block, &info.sender, action)?;
            Ok(Response::new().add_attribute("action", "update_ownership"))
        }
        
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenInfo {} => to_json_binary(&TOKEN_INFO.load(deps.storage)?),
        QueryMsg::Balance { address } => {
            let addr =
                validate_addr(deps, &address).map_err(|e| StdError::generic_err(e.to_string()))?;

            let balance = BALANCES.may_load(deps.storage, &addr)?.unwrap_or(0);
            to_json_binary(&balance)
        }
    }
}

