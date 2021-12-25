use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

near_sdk::setup_alloc!();

const MIN_ATTACHED_BALANCE: Balance = 0;

// const NFT_GAS_NEW: GAS = 4_201_706_103_318_740_100_000_000;

const NFT_WASM_CODE: &[u8] = include_bytes!("../../SimpleNFT/res/non_fungible_token.wasm");

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

    Promise::new(subaccount_id)
        .create_account()
        .transfer(env::attached_deposit())
        .add_full_access_key(env::signer_account_pk())
        .deploy_contract(code)
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