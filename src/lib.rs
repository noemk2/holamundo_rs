use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    value: u64,
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    pub fn hello(&self) -> &str {
        return "Hello world";
    }

    pub fn say_hello(&self) -> String {
        let account_id = env::signer_account_id();
        return "Hello ".to_owned() + &account_id.to_string();
    }
}
