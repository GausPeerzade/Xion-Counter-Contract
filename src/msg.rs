use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub count: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    IncrementCounter {},
    DicrementCounter {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetConfigResponse)]
    GetConfig {},
}

#[cw_serde]
pub struct GetConfigResponse {
    pub owner: Addr,
    pub current_count: Uint128,
}
