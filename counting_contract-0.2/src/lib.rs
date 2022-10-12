#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, StdResult, Response, Binary, to_binary, Empty};
use error::ContractError;
use msg::InstantiateMsg;

mod contract;
pub mod msg;
pub mod error;
mod state;
#[cfg(any(test, feature = "tests"))]
pub mod multitest;

// constructor for smart contract
#[cfg_attr(not(feature = "library"),entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: msg::QueryMsg,
) -> StdResult<Binary> {
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_binary(&contract::query::value(deps)?),
    }
}

#[cfg_attr(not(feature = "library"),entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::ExecMsg,
) -> Result<Response, ContractError> {
    use msg::ExecMsg::*;

    match msg {
        Donate {} => contract::exec::donate(deps, info).map_err(ContractError::from),
        Withdraw {} => contract::exec::withdraw(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"),entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    contract::migrate(deps)
}