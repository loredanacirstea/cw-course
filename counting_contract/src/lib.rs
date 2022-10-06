use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Empty, StdResult, Response, entry_point, Binary, to_binary};

mod contract;
pub mod msg;
mod state;

// constructor for smart contract
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    contract::instantiate(deps)
}

#[entry_point]
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

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: msg::ExecMsg,
) -> StdResult<Response> {
    use msg::ExecMsg::*;

    match msg {
        Poke {} => contract::exec::poke(deps, info),
    }
}

#[cfg(test)]
mod test {
    use crate::msg::{QueryMsg, ValueResp, ExecMsg};

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
            &QueryMsg::Value{}, 
            &[], 
            "Counting", 
            None,
        ).unwrap();
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value{})
            .unwrap();
        assert_eq!(resp, ValueResp { value: 0});
    }

    #[test]
    fn poke() {
        let mut app = App::default();
        let sender = Addr::unchecked("sender");
        let contract_id = app.store_code(counting_contract());
        let contract_addr = app.instantiate_contract(
            contract_id, 
            sender.clone(),
            &QueryMsg::Value{}, 
            &[], 
            "Counting", 
            None,
        ).unwrap();

        app.execute_contract(sender.clone(), contract_addr.clone(), &ExecMsg::Poke {}, &[])
            .unwrap();
        app.execute_contract(sender, contract_addr.clone(), &ExecMsg::Poke {}, &[])
            .unwrap();
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value{})
            .unwrap();
        assert_eq!(resp, ValueResp { value: 2});
    }
}