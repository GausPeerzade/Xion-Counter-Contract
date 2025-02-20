use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

pub const COUNTER: Item<Uint128> = Item::new("counter");
pub const OWNER: Item<Addr> = Item::new("admin");
