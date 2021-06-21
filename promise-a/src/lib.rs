use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, log, metadata, near_bindgen, setup_alloc, AccountId};
use near_sdk::serde_json;
use near_sdk::json_types::Base64VecU8;

use std::collections::HashMap;

setup_alloc!();

pub const SINGLE_CALL_GAS: u64 = 20_000_000_000_000; // 2 x 10^14

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn set_message(&self, account_a: AccountId) -> String {
        log!(
            "Method: set_message. I am {}. Called by {}.",
            env::current_account_id(),
            env::predecessor_account_id(),
        );

        log!("Param account_a {}", account_a);

        let prepaid_gas = env::prepaid_gas();
        log!("prepaid_gas {}", prepaid_gas);

        ext_promise_a::get_message(
            &account_a,
            1,
            SINGLE_CALL_GAS
        )
        .then(ext_self::on_data(
            &env::current_account_id(),
            0,
            SINGLE_CALL_GAS
        ));

        log!("after ext call");

        "World".to_string()
    }

    pub fn get_message(&self, message: String) -> String {
        log!("Method: get_message. I am {}. Called by {}", env::current_account_id(), env::predecessor_account_id());

        // "Hello".to_string()
        message
    }

    pub fn on_data(
        &mut self,
        #[callback] value_of_a: String,
    ) -> String {
        log!(
            "Method: on_data. I am {}. Called by {}. value_of_a {}",
            env::current_account_id(),
            env::predecessor_account_id(),
            value_of_a,
        );

        format!("on_data {}", value_of_a)
    }

    fn get_random(&self) -> String {
        let random = env::random_seed();
        log!("random {:?}", random);
        serde_json::to_string(&Base64VecU8(random)).unwrap()
    }
}

#[ext_contract(ext_promise_a)]
pub trait ExtPromiseA {
    fn set_message(&self, account_a: AccountId) -> String;
    fn get_message(&self) -> String;
}

#[ext_contract(ext_self)]
pub trait ExtContract {
    fn on_data(
        &mut self,
        #[callback] value_of_a: String,
    ) -> String;
}
