use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, log, metadata, near_bindgen, setup_alloc, AccountId, Promise};

setup_alloc!();

pub const SINGLE_CALL_GAS: u64 = 20_000_000_000_000; // 2 x 10^14

pub const FINISH_CALL_A_GAS: u64 = 30_000_000_000_000 + SINGLE_CALL_GAS;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn call_a(&mut self, account_a: String, account_a2: String) {
        log!(
            "Method: call_a. I am {}. Called by {}. call_a",
            env::current_account_id(),
            env::predecessor_account_id(),
        );

        log!("Param account_a {}, account_a2 {}", account_a, account_a2);

        let prepaid_gas = env::prepaid_gas();
        log!("prepaid_gas {}", prepaid_gas);

        // ext_promise_a::get_message(
        //     &account_a,
        //     1,
        //     SINGLE_CALL_GAS
        // )
        // .then(ext_self::on_data(
        //     account_a,
        //     &env::current_account_id(),
        //     0,
        //     SINGLE_CALL_GAS
        // ));

        ext_promise_a::get_message(
            "Hello".to_string(),
            &account_a,
            1,
            SINGLE_CALL_GAS
        )
        .then(ext_self::finish_call_a(
            account_a2,
            &env::current_account_id(),
            1,
            FINISH_CALL_A_GAS
        ))
        .then(ext_self::on_data(
            account_a,
            &env::current_account_id(),
            0,
            SINGLE_CALL_GAS
        ));

        // ext_promise_a::set_message(
        //     account_a2,
        //     &account_a,
        //     1,
        //     prepaid_gas
        // )
        // .then(ext_self::on_data(
        //     account_a,
        //     &env::current_account_id(),
        //     0,
        //     prepaid_gas
        // ));

        log!("after ext call");
        env::value_return(b"Test");
    }

    pub fn finish_call_a(
        &self,
        #[callback]
        message: String,
        account_a: AccountId,
    ) -> Promise {
        log!(
            "Method: finish_call_a. I am {}. Called by {}. message {}, account_a {}",
            env::current_account_id(),
            env::predecessor_account_id(),
            message,
            account_a
        );

        ext_promise_a::get_message(
            "World".to_string(),
            &account_a,
            1,
            SINGLE_CALL_GAS
        )
    }

    pub fn on_data(
        &mut self,
        account_a: AccountId,
        #[callback] value_of_a: String,
    ) -> String {
        log!(
            "Method: on_data. I am {}. Called by {}. value_of_a {}",
            env::current_account_id(),
            env::predecessor_account_id(),
            value_of_a,
        );

        format!("on_data {}-{}", account_a, value_of_a)
    }
}

#[ext_contract(ext_promise_a)]
pub trait ExtPromiseA {
    fn set_message(&self, account_a: AccountId) -> String;
    fn get_message(&self, message: String) -> String;
}

#[ext_contract(ext_self)]
pub trait ExtContract {
    fn on_data(
        &mut self,
        account_a: AccountId,
        #[callback] value_of_a: String,
    ) -> String;

    fn finish_call_a(
        &self,
        #[callback]
        message: String,
        account_a: AccountId,
    ) -> Promise;
}

