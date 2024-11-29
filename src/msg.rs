use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    IncrementCounter {},
    DicrementCounter {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
