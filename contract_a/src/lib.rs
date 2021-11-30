/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

use near_sdk::{AccountId, Balance, callback, env, ext_contract, Gas, near_bindgen, Promise, PromiseOrValue, PromiseResult, setup_alloc};
// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};

setup_alloc!();

const NO_DEPOSIT: Balance = 0;
// const BASE_GAS: Gas = 20_000_000_000_000;
const BASE_GAS: Gas = 2428055156040;
const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BalanceExt {
    account_id: AccountId,
    balance: U128,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractA {}

impl Default for ContractA {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl ContractA {
    pub fn get_balance(&self) -> Balance {
        env::account_balance()
    }
    pub fn update_balance(&mut self, account_id: AccountId) -> Balance {
        0
    }
    pub fn call_balance_ext(&mut self, receiver_id: AccountId) -> Promise {
        ext_contract_b::get_balance(&receiver_id, NO_DEPOSIT, BASE_GAS).then(
            ext_self::callback_promise_result(
                &env::current_account_id(),
                0,
                BASE_GAS,
            )
        )
    }
    pub fn transfer_amount(&self) -> Promise {
        let amount = U128::from(ONE_NEAR * 2);
        Promise::new("challenge9-b.3ugen.testnet".to_string()).transfer(Balance::from(amount))
    }


    #[private]
    pub fn callback_promise_result(&mut self) -> PromiseOrValue<String> {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        env::log(b"callback result");
        match env::promise_result(0) {
            PromiseResult::NotReady => PromiseOrValue::Value("not ready".to_string()),
            PromiseResult::Successful(val) => {
                env::log(format!("result: {:?}", val).as_bytes());
                if let Ok(balance_ext) = near_sdk::serde_json::from_slice::<BalanceExt>(&val) {
                    let amount = U128::from(ONE_NEAR * 2);
                    let balance = Balance::from(balance_ext.balance);
                    env::log(format!("get balance: {}", balance).as_bytes());
                    if balance < 15 * ONE_NEAR {
                        env::log(b"send some near");
                        PromiseOrValue::Promise(Promise::new(balance_ext.account_id).transfer(Balance::from(amount)))
                    } else {
                        PromiseOrValue::Value("wallet is full".to_string())
                    }
                } else {
                    env::log(b"can't get balance from wallet");
                    PromiseOrValue::Value("can't get balance from wallet".to_string())
                }
            }
            PromiseResult::Failed => PromiseOrValue::Value("err call failed".to_string())
        }
    }
}

#[ext_contract(ext_contract_b)]
trait ContractB {
    fn get_balance(&self) -> BalanceExt;
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_promise_result() -> bool;
}
/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use near_sdk::{testing_env, VMContext};
    use near_sdk::MockedBlockchain;

    use super::*;

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ContractA::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.get_greeting("bob_near".to_string())
        );
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = ContractA::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            "Hello".to_string(),
            contract.get_greeting("francis.near".to_string())
        );
    }
}
