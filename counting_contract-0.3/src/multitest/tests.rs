use cosmwasm_std::{Addr, Coin, Empty, coins, Decimal};
use cw_multi_test::{App, Contract, ContractWrapper};

use crate::msg::Parent;
use crate::{execute, instantiate, query, multitest::CountingContract, error::ContractError};
use crate::state::{STATE, State};
use counting_contract_0_1_0::multitest::CountingContract as CountingContract_0_1_0;

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
    let sender = Addr::unchecked("sender");
    let contract_id = app.store_code(counting_contract());
    
    let contract = CountingContract::instantiate(
        &mut app, 
        contract_id,
        &sender, 
        None,
        "Counting contract", 
        Coin::new(10, ATOM),
        None,
    ).unwrap();
    
    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value,0);
}

#[test]
fn donate() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");
    let contract_id = app.store_code(counting_contract());
    let contract = CountingContract::instantiate(
        &mut app, 
        contract_id,
        &sender, 
        None,
        "Counting contract", 
        Coin::new(10, ATOM),
        None,
    ).unwrap();

    contract.donate(&mut app, &sender, &[]).unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value,0);
}

#[test]
fn donate_with_funds() {
    let sender = Addr::unchecked("sender");
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, ATOM))
            .unwrap();
    });
    
    let contract_id = app.store_code(counting_contract());
    let contract = CountingContract::instantiate(
        &mut app, 
        contract_id,
        &sender, 
        None,
        "Counting contract", 
        Coin::new(10, ATOM),
        None,
    ).unwrap();

    contract.donate(&mut app, &sender, &coins(10, ATOM)).unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value, 1);

    assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
    assert_eq!(app.wrap().query_all_balances(contract.addr()).unwrap(), coins(10, ATOM));

}

#[test]
fn withdraw() {
    let owner = Addr::unchecked("owner");
    let sender1 = Addr::unchecked("sender1");
    let sender2 = Addr::unchecked("sender2");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, coins(10, ATOM))
            .unwrap();
        router
            .bank
            .init_balance(storage, &sender2, coins(5, ATOM))
            .unwrap();
    });
    
    let contract_id = app.store_code(counting_contract());
    let contract = CountingContract::instantiate(
        &mut app, 
        contract_id,
        &owner, 
        None,
        "Counting contract", 
        Coin::new(10, ATOM),
        None,
    ).unwrap();

    contract.donate(&mut app, &sender1, &coins(10, ATOM)).unwrap();

    contract.donate(&mut app, &sender2, &coins(5, ATOM)).unwrap();

    contract.withdraw(&mut app, &owner).unwrap();

    assert_eq!(app.wrap().query_all_balances(owner).unwrap(), coins(15, ATOM));
    assert_eq!(app.wrap().query_all_balances(contract.addr()).unwrap(), vec![]);
    assert_eq!(app.wrap().query_all_balances(sender1).unwrap(), vec![]);
    assert_eq!(app.wrap().query_all_balances(sender2).unwrap(), vec![]);

}

#[test]
fn unauthorized_withdraw() {
    let owner = Addr::unchecked("owner");
    let member = Addr::unchecked("member");

    let mut app = App::default();
    
    let contract_id = app.store_code(counting_contract());
    let contract = CountingContract::instantiate(
        &mut app, 
        contract_id,
        &owner, 
        None,
        "Counting contract", 
        Coin::new(10, ATOM),
        None,
    ).unwrap();

    let err = contract.withdraw(&mut app, &member).unwrap_err();

    assert_eq!(
        err,
        ContractError::Unauthorized {owner: owner.into()}
    );

}

#[test]
fn migration() {
    let owner = Addr::unchecked("owner");
    let admin = Addr::unchecked("admin");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, ATOM))
            .unwrap();
    });

    let old_code_id = CountingContract_0_1_0::store_code(&mut app);
    let new_code_id = CountingContract::store_code(&mut app);

    let contract = CountingContract_0_1_0::instantiate(
        &mut app, 
        old_code_id,
        &owner, 
        Some(&admin),
        "Counting contract", 
        Coin::new(10, ATOM),
    ).unwrap();

    contract
        .donate(&mut app, &sender, &coins(10, ATOM))
        .unwrap();
    
    let contract = CountingContract::migrate(&mut app, &admin, contract.addr(), new_code_id, None).unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value, 1);

    let state = STATE.query(&app.wrap(), contract.addr().clone()).unwrap();
    assert_eq!(
        state,
        State {
            counter: 1,
            minimal_donation: Coin::new(10, ATOM),
            donating_parent: None,
        }
    );
}

#[test]
fn migration_no_update() {
    let owner = Addr::unchecked("owner");
    let admin = Addr::unchecked("admin");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);

    let contract = CountingContract::instantiate(
        &mut app, 
        code_id,
        &owner, 
        Some(&admin),
        "Counting contract", 
        Coin::new(10, ATOM),
        None,
    ).unwrap();

    CountingContract::migrate(&mut app, &admin, contract.addr(), code_id, None).unwrap();
}

#[test]
fn donating_parent() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(20, ATOM))
            .unwrap();
    });

    let code_id = CountingContract::store_code(&mut app);
    let contract_parent = CountingContract::instantiate(
        &mut app, 
        code_id, 
        &owner, 
        None,
        "counting contract",
        Coin::new(10, ATOM),
        None,
    ).unwrap();

    let contract = CountingContract::instantiate(
        &mut app, 
        code_id, 
        &owner, 
        None,
        "counting contract",
        Coin::new(10, ATOM),
        Some(Parent {
            addr: contract_parent.addr().to_string(),
            donating_period: 2,
            part: Decimal::percent(10),
        }),
    ).unwrap();

    contract.donate(&mut app, &sender, &coins(10, ATOM)).unwrap();

    assert_eq!(app.wrap().query_all_balances(sender.clone()).unwrap(), coins(10, ATOM));
    assert_eq!(app.wrap().query_all_balances(contract.addr()).unwrap(), coins(10, ATOM));
    assert_eq!(app.wrap().query_all_balances(contract_parent.addr()).unwrap(), vec![]);
    assert_eq!(app.wrap().query_all_balances(owner.clone()).unwrap(), vec![]);

    contract.donate(&mut app, &sender, &coins(10, ATOM)).unwrap();

    assert_eq!(app.wrap().query_all_balances(sender.clone()).unwrap(), vec![]);
    assert_eq!(app.wrap().query_all_balances(contract.addr()).unwrap(), coins(18, ATOM));
    assert_eq!(app.wrap().query_all_balances(contract_parent.addr()).unwrap(), coins(2, ATOM));
    assert_eq!(app.wrap().query_all_balances(owner.clone()).unwrap(), vec![]);

}