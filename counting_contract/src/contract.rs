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