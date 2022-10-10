use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, StdResult, Response, entry_point, Binary, to_binary};
use msg::InstantiateMsg;

mod contract;
pub mod msg;
mod state;

// constructor for smart contract
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, msg)
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
        Donate {} => contract::exec::donate(deps, info),
    }
}

#[cfg(test)]
mod test {
    use crate::msg::{QueryMsg, ValueResp, ExecMsg};

    use super::*;

    use cosmwasm_std::{Empty, Addr, Coin, coins};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor, AppBuilder};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            execute,
            instantiate,
            query,
        );
        Box::new(contract)
    }

    const ATOM: &str = "atom";

    #[test]
    fn query_value() {
        let mut app = App::default();
        let contract_id = app.store_code(counting_contract());
        let contract_addr = app.instantiate_contract(
            contract_id, 
            Addr::unchecked("sender"),
            &InstantiateMsg{minimal_donation: Coin::new(10, ATOM)}, 
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
    fn donate() {
        let mut app = App::default();
        let sender = Addr::unchecked("sender");
        let contract_id = app.store_code(counting_contract());
        let contract_addr = app.instantiate_contract(
            contract_id, 
            sender.clone(),
            &InstantiateMsg{minimal_donation: Coin::new(10, ATOM)}, 
            &[], 
            "Counting", 
            None,
        ).unwrap();

        app.execute_contract(sender.clone(), contract_addr.clone(), &ExecMsg::Donate {}, &[])
            .unwrap();
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value{})
            .unwrap();
        assert_eq!(resp, ValueResp { value: 0});
    }

    #[test]
    fn donate_with_funds() {
        let sender = Addr::unchecked("sender");
        let mut app = AppBuilder::new().build(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, ATOM))
                .unwrap();
        });
        
        let contract_id = app.store_code(counting_contract());
        let contract_addr = app.instantiate_contract(
            contract_id, 
            sender.clone(),
            &InstantiateMsg{minimal_donation: Coin::new(10, ATOM)}, 
            &[], 
            "Counting", 
            None,
        ).unwrap();

        app.execute_contract(sender.clone(), contract_addr.clone(), &ExecMsg::Donate {}, &coins(10, ATOM))
            .unwrap();
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Value{})
            .unwrap();
        assert_eq!(resp, ValueResp { value: 1});

        assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
        assert_eq!(app.wrap().query_all_balances(contract_addr).unwrap(), coins(10, ATOM));

    }
}