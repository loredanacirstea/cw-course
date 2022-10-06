use cosmwasm_std::{DepsMut, StdResult, Response};

use crate::state::COUNTER;

pub fn instantiate(deps: DepsMut) -> StdResult<Response> {
    COUNTER.save(deps.storage, &0)?;
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::{msg::ValueResp, state::COUNTER};

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value})
    }
}

pub mod exec {
    use cosmwasm_std::{DepsMut, StdResult, Response, MessageInfo};

    use crate::state::COUNTER;

    pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let value = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &value)?;
        // COUNTER.update(deps.storage, |counter| -> StdResult<_> {Ok(counter + 1)})?;

        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", value.to_string());
        
        Ok(resp)
    }
}