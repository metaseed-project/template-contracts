use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, Gas};
use near_sdk::serde::{Serialize};
use near_sdk::serde_json;


near_sdk::setup_alloc!();

const MIN_ATTACHED_BALANCE: Balance = 0;

const NFT_GAS_NEW: Gas = 50_000_000_000_000;

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

    assert!(
      is_valid_symbol(&prefix),
      "Prefix is invalid"
    );

    let subaccount_id = 
      format!("{}.{}", prefix, env::current_account_id());
    assert!(
      env::is_valid_account_id(subaccount_id.as_bytes()),
      "Token Account ID is invalid"
    );

    assert!(
      env::attached_deposit() >= MIN_ATTACHED_BALANCE,
      "Not enough attached deposit"
    );

    let args: TokenArgs = TokenArgs {
      owner_id: env::predecessor_account_id(),
    };

    Promise::new(subaccount_id)
        .create_account()
        .transfer(env::attached_deposit())
        .add_full_access_key(env::signer_account_pk())
        .deploy_contract(code)
        .function_call(
          b"new_default_meta".to_vec(),
          serde_json::to_vec(&args).unwrap(),
          0,
          NFT_GAS_NEW
        )
}


fn is_valid_symbol(token_id: &str) -> bool {
  for c in token_id.as_bytes() {
      match c {
          b'0'..=b'9' | b'a'..=b'z' | b'_' | b'-' => (),
          _ => return false,
      }
  }
  true
}