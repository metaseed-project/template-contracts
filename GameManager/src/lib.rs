use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, Gas};
use near_sdk::serde::{Serialize};
use near_sdk::serde_json;

near_sdk::setup_alloc!();

const INITIAL_INGAME_BALANCE: Balance = 3_200_000_000_000_000_000_000_000; // 3e24yN, 3.2N
const GAS: Gas = 50_000_000_000_000;
                  2_428_021_617_030

const NFT_WASM_CODE: &[u8] = include_bytes!("../../SimpleNFT/res/non_fungible_token.wasm");

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenArgs {
    pub owner_id: AccountId,
}

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct GameManager {
}

#[near_bindgen]
impl GameManager {

    #[payable]
    pub fn create_ingame_nft(prefix: AccountId) {
        create_ingame_contract(prefix, NFT_WASM_CODE.to_vec());
    }
}


fn create_ingame_contract(prefix: AccountId, code: Vec<u8>) -> Promise {
    let subaccount_id = 
      format!("{}.{}", prefix, env::current_account_id());
    assert!(
      env::is_valid_account_id(subaccount_id.as_bytes()),
      "Token Account ID is invalid"
    );

    let args: TokenArgs = TokenArgs {
      owner_id: env::predecessor_account_id(),
    };

    Promise::new(subaccount_id)
        .create_account()
        .add_full_access_key(env::signer_account_pk())
        .transfer(INITIAL_INGAME_BALANCE)
        .deploy_contract(code)
        .function_call(b"new_default_meta".to_vec(), serde_json::to_vec(&args).unwrap(), 0, GAS)
}