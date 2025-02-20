#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{self, ExecuteMsg, GetConfigResponse, InstantiateMsg, QueryMsg};
use crate::state::{COUNTER, OWNER};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw_counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.save(deps.storage, &msg.owner)?;
    COUNTER.save(deps.storage, &msg.count)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::IncrementCounter {} => execute_increment(deps, env, info),
        ExecuteMsg::DicrementCounter {} => execute_decrement(deps, env, info),
    }
}

fn execute_increment(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let user = info.sender;
    let owner = OWNER.load(deps.storage)?;
    let counter = COUNTER.load(deps.storage).unwrap_or_default();

    if user != owner {
        return Err(ContractError::NotAdmin {});
    }

    let new_value = counter + Uint128::from(1_u128);

    COUNTER.save(deps.storage, &new_value)?;
    Ok(Response::new())
}

fn execute_decrement(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let user = info.sender;
    let owner = OWNER.load(deps.storage)?;
    let counter = COUNTER.load(deps.storage).unwrap_or_default();

    if user != owner {
        return Err(ContractError::NotAdmin {});
    }

    let new_value = counter - Uint128::one();

    COUNTER.save(deps.storage, &new_value)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&get_config_query(deps)?),
    }
}

fn get_config_query(deps: Deps) -> StdResult<GetConfigResponse> {
    let owner = OWNER.load(deps.storage)?;
    let counter = COUNTER.load(deps.storage)?;

    let config_response: GetConfigResponse = GetConfigResponse {
        owner: owner,
        current_count: counter,
    };

    Ok(config_response)
}

#[cfg(test)]
mod tests {}
