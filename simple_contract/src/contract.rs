use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::state::{COUNTER};

// const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut,
    _info: MessageInfo,
) -> StdResult<Response> {
    // set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    COUNTER.save(deps.storage, &2)?;
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::msg::ValueResp;
    use crate::state::COUNTER;

    pub fn value_const(_deps: Deps) -> StdResult<ValueResp> {
        Ok(ValueResp { value: 5 })
    }

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
}

pub mod exec {
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

    use crate::state::{COUNTER};

    pub fn increase(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let mut counter = COUNTER.load(deps.storage)?;
        counter += 1;
        COUNTER.save(deps.storage, &counter)?;

        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }
}
