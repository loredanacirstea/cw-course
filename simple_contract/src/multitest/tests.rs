use cosmwasm_std::{Addr};
use cw_multi_test::App;

use crate::msg::ValueResp;

use super::contract::CountingContract;

#[test]
fn query_value() {
    let owner = Addr::unchecked("owner");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
    )
    .unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 2 });
}

#[test]
fn increase() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
    )
    .unwrap();

    contract.increase(&mut app, &sender).unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 3 });
}
