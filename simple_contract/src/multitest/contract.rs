use cosmwasm_std::{Addr, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResp};
use crate::{execute, instantiate, query};

pub struct CountingContract(Addr);

impl CountingContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate<'a>(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        admin: impl Into<Option<&'a Addr>>,
    ) -> StdResult<Self> {
        let admin = admin.into();

        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
            },
            &[],
            label,
            admin.map(Addr::to_string),
        )
        .map(CountingContract)
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn increase(
        &self,
        app: &mut App,
        sender: &Addr,
    ) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Increase {}, &[])
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    #[track_caller]
    pub fn query_value(&self, app: &App) -> StdResult<ValueResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Value {})
    }
}

impl From<CountingContract> for Addr {
    fn from(contract: CountingContract) -> Self {
        contract.0
    }
}
