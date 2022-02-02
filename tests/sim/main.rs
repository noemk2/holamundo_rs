pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};
// use near_sdk::serde_json::json;
use near_sdk::serde_json::Value;

//
use near_sdk_sim::{
    call, deploy, init_simulator, view, ContractAccount, UserAccount, 
};

use simulation::ContractContract;
// use rust_counter_tutorial::CounterContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_BYTES => "res/holamundo_rs.wasm",
}

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;


fn init() -> (UserAccount, ContractAccount<ContractContract>) {
    // let firmante = UserAccount::signer;
    // println!("Este es el Firmante {}", firmante);

    let root = init_simulator(None);

    // Deploy the compiled Wasm bytes
    let hola: ContractAccount<ContractContract> = deploy!(
        contract: ContractContract,
        contract_id: "hola".to_string(),
        bytes: &COUNTER_BYTES,
        signer_account: root
    );

    (root, hola)
}

#[test]
fn simulate_helloword() {

    let (root, hola) = init();

    //Get "Hello world"
    let hi = "Hello world".to_string();
    let current_str= view!(hola.hello_world()).unwrap_json_value();
    println!("The word is: {}", &current_str.to_string());
    assert_eq!(Value::String(hi.clone()), current_str);


    // call signer name 
    let ba = "Hello root".to_string();
    let saludo = call!(root, hola.hello()).unwrap_json_value();
    println!("The word is: {}", &saludo.to_string());
    assert_eq!(Value::String(ba.clone()), saludo);
}
