use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn hello_world(&self) -> &str {
        return "Hello world";
    }

    pub fn hello(&self) -> String {
        let account_id = env::signer_account_id();
        return "Hello ".to_owned() + &account_id.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
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
    fn hello_world() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Contract {};
        assert_eq!("Hello world", contract.hello_world());
    }

    #[test]
    fn hello() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Contract {};
        let account_id = env::signer_account_id();
        assert_eq!(
            "Hello ".to_owned() + &account_id.to_string(),
            contract.hello()
        );
    }
}
