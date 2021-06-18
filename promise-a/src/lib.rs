use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, metadata, near_bindgen, setup_alloc};

use std::collections::HashMap;

setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn get_message(&self) -> String {
        log!("I am {}. Called by {}", env::current_account_id(), env::predecessor_account_id());

        "Hello".to_string()
    }
}
