use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Empty, StdResult, Response, entry_point, Binary, to_binary};

mod contract;
pub mod msg;

// constructor for smart contract
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(
    _deps: Deps,
    _env: Env,
    msg: msg::QueryMsg,
) -> StdResult<Binary> {
    use msg::QueryMsg::*;

    match msg {
        Value {value} => to_binary(&contract::query::value(value)),
    }
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg(test)]
mod test {
    use crate::msg::{QueryMsg, ValueResp};

    use super::*;

    use cosmwasm_std::{Empty, Addr};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            execute,
            instantiate,
            query,
        );
        Box::new(contract)
    }

    #[test]
    fn query_value() {
        let mut app = App::default();
        let contract_id = app.store_code(counting_contract());
        let contract_addr = app.instantiate_contract(
            contract_id, 
            Addr::unchecked("sender"),
            &QueryMsg::Value{value: 4}, 
            &[], 
            "Counting", 
            None,
        ).unwrap();
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value{value: 4})
            .unwrap();
        assert_eq!(resp, ValueResp { value: 5});
    }
}