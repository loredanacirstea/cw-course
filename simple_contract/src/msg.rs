use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},

    #[returns(ValueResp)]
    ValueConst {},
}

#[cw_serde]
pub enum ExecMsg {
    Increase {},
}

#[cw_serde]
pub struct ValueResp {
    pub value: u64,
}
