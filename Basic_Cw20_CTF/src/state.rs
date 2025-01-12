// @dev This file is a critical component of the CosmWasm smart contract, as it defines how data is structured and stored. Basically all the storage will happen here

use schemars::JsonSchema; //schemars is a Rust crate that generates JSON schemas.

use serde::{Deserialize, Serialize}; //	Serialize and Deserialize derive macros allow the State struct to be converted to/from a format that can be stored on the blockchain.

use cosmwasm_std::{Addr, Decimal}; //Addr is a type provided by CosmWasm to represent validated blockchain addresses. Unlike simple strings, Addr ensures that the address conforms to the blockchain’s address format.

use cw_storage_plus::{Item, Map};
// cw-storage-plus is a CosmWasm helper crate for working with persistent storage. Item is a high-level abstraction for a single piece of data stored on the blockchain.

// Store Token MetaData
pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");

//Storing Account Balances Mapping
pub const BALANCES: Map<&Addr, u128> = Map::new("balances");

//ALlowances
pub const ALLOWANCES: Map<(&Addr, &Addr), u128> = Map::new("allowances");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub circulating_supply: u128,
    pub max_supply: u128,
    pub owner: Addr,         // Owner of the token
    pub fee_collector: Addr, // Address to collect fees
    pub fee_rate: Decimal,
}

// // What’s Happening Here:
// // 	This line defines STATE as a Singleton storage variable using the cosmwasm_std::singleton helper.
// // 	"state":
// // 	    • This is the storage key used to save/retrieve the State struct from the blockchain’s key-value store.
// // 	• Singleton<State>:
// // 	    • A Singleton is a storage abstraction for managing a single data item (in this case, the State struct).
// // 	• Why You Need It:
// // 	    • This constant defines how and where the State struct is stored persistently.
// // 	•	By using a Singleton:
// // 	    • Only one instance of State exists in storage.
// // 	You can easily save, update, and retrieve the state.

// pub const STATE: cosmwasm_std::Singleton<State> = cosmwasm_std::singleton("state");
