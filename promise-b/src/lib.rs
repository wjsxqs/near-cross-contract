use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, log, metadata, near_bindgen, setup_alloc, AccountId};

use std::collections::HashMap;

setup_alloc!();

pub const SINGLE_CALL_GAS: u64 = 20_000_000_000_000; // 2 x 10^14

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn call_a(&mut self, account_a: String, account_a2: String) {
        let sender = env::predecessor_account_id();
        log!("sender {}", sender);

        ext_promise_a::get_message(
            &account_a,
            1,
            SINGLE_CALL_GAS
        )
        .and(ext_promise_a::get_message(
            &account_a2,
            1,
            SINGLE_CALL_GAS
        ))
        .then(ext_self::on_data(
            &env::current_account_id(),
            0,
            SINGLE_CALL_GAS
        ));

        log!("after call");
        env::value_return(b"Test");
    }

    pub fn on_data(
        &mut self,
        #[callback] value_of_a: String,
        #[callback] value_of_a2: String
    ) -> String {
        log!(
            "I am {}. Called by {}. on_data: a {}, a2 {}",
            env::current_account_id(),
            env::predecessor_account_id(),
            value_of_a,
            value_of_a2,
        );

        format!("on_data {}-{}", value_of_a, value_of_a2)
    }
}

#[ext_contract(ext_promise_a)]
pub trait ExtPromiseA {
    fn get_message(&self) -> String;
}

#[ext_contract(ext_self)]
pub trait ExtContract {
    fn on_data(
        &mut self,
        #[callback] value_of_a: String,
    ) -> String;
}

