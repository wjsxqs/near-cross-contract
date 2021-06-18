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
    pub fn call_a(&mut self, account_a: String) {
        let sender = env::predecessor_account_id();
        log!("sender {}", sender);

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

        // let result = ext_promise_a::get_message(
        //     &account_a,
        //     1,
        //     SINGLE_CALL_GAS
        // );

        // log!("result {}", result);

        log!("after call");
        env::value_return(b"Test");
    }

    pub fn on_data(
        &mut self,
        #[callback] value_of_a: String,
        #[callback] value_of_c: String,
    ) -> String {
        log!(
            "I am {}. Called by {}. on_data: a {}, c {}",
            env::current_account_id(),
            env::predecessor_account_id(),
            value_of_a,
            value_of_c
        );

        format!("on_data {}.{}", value_of_a, value_of_c)
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

