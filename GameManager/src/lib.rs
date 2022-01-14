use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, Gas};
use near_sdk::serde::{Serialize};
use near_sdk::serde_json;


near_sdk::setup_alloc!();

const MIN_NFT_ATTACHED_BALANCE: Balance = 0;
const NFT_GAS_NEW: Gas = 50_000_000_000_000;
const NFT_WASM_CODE: &[u8] = include_bytes!("../../SimpleNFT/res/non_fungible_token.wasm");

const MIN_FT_ATTACHED_BALANCE: Balance = 0;
const FT_GAS_NEW: Gas = 50_000_000_000_000;
const FT_WASM_CODE: &[u8] = include_bytes!("../../SimpleFT/res/fungible_token.wasm");

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FTMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u128,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FTArgs {
    pub owner_id: AccountId,
    pub total_supply: u128,
    pub metadata: FTMetadata,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTArgs {
    pub owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetOptions {
  extra: String,
}

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct GameManager {
    pub owner_id: AccountId,
    pub ingame_assets: UnorderedMap<AccountId, AssetOptions>,
}

#[near_bindgen]
impl GameManager {

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
          owner_id,
          ingame_assets: UnorderedMap::new(b"r".to_vec()),
        }
    }

    #[payable]
    pub fn create_ingame_nft(&mut self, prefix: AccountId) {
        assert!(
          env::predecessor_account_id() == self.owner_id,
          "Not an game owner"
        );

        let subaccount_id = create_asset_subaccount(prefix);

        let options: AssetOptions = AssetOptions {
          extra: "".to_string(),
        };

        self.ingame_assets.insert(&subaccount_id, &options);
        
        assert!(
          env::attached_deposit() >= MIN_NFT_ATTACHED_BALANCE,
          "Not enough attached deposit"
        );
    
        let args: NFTArgs = NFTArgs {
          owner_id: env::predecessor_account_id(),
        };
    
        Promise::new(subaccount_id)
            .create_account()
            .transfer(env::attached_deposit())
            .add_full_access_key(env::signer_account_pk())
            .deploy_contract(NFT_WASM_CODE.to_vec())
            .function_call(
              b"new_default_meta".to_vec(),
              serde_json::to_vec(&args).unwrap(),
              0,
              NFT_GAS_NEW
            );
    }

    #[payable]
    pub fn create_ingame_ft(&mut self, prefix: AccountId, name: String, symbol: String, total_supply: u128) {
        assert!(
          env::predecessor_account_id() == self.owner_id,
          "Not an game owner"
        );

        let subaccount_id = create_asset_subaccount(prefix);

        let options: AssetOptions = AssetOptions {
          extra: "".to_string(),
        };

        self.ingame_assets.insert(&subaccount_id, &options);
        
        assert!(
          env::attached_deposit() >= MIN_NFT_ATTACHED_BALANCE,
          "Not enough attached deposit"
        );
    
        let metadata: FTMetadata = FTMetadata {
          spec: "ft-1.0.0".to_string(),
          name: name,
          symbol: symbol,
          decimals: 8,
        };

        let args: FTArgs = FTArgs {
          owner_id: env::predecessor_account_id(),
          total_supply: total_supply,
          metadata: metadata,
        };
    
        Promise::new(subaccount_id)
            .create_account()
            .transfer(env::attached_deposit())
            .add_full_access_key(env::signer_account_pk())
            .deploy_contract(NFT_WASM_CODE.to_vec())
            .function_call(
              b"new".to_vec(),
              serde_json::to_vec(&args).unwrap(),
              0,
              NFT_GAS_NEW
            );
    }

    #[payable]
    pub fn add_asset(&mut self, account_id: AccountId, extra: String) {
      assert!(
        env::predecessor_account_id() == self.owner_id,
        "Not an game owner"
      );

      assert!(
        env::is_valid_account_id(account_id.as_bytes()),
        "Token Account ID is invalid"
      );

      let options: AssetOptions = AssetOptions {
        extra: extra,
      };

      self.ingame_assets.insert(&account_id, &options);
    }

    pub fn get_asset(&self, account_id: AccountId) -> Option<AssetOptions> {
      return self.ingame_assets.get(&account_id);
    }

    pub fn get_counts(&self) -> u64 {
      return self.ingame_assets.len();
    }

    /// Retrieves multiple elements from the `ingame_assets`.
    pub fn get_assets(&self, from_index: u64, limit: u64) -> Vec<(AccountId, AssetOptions)> {
      let keys = self.ingame_assets.keys_as_vector();
      let values = self.ingame_assets.values_as_vector();
      (from_index..std::cmp::min(from_index + limit, self.ingame_assets.len()))
          .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap()))
          .collect()
    }
}

fn create_asset_subaccount(prefix: AccountId) -> String {
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

  subaccount_id
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