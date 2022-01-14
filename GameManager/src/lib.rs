use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::serde::{Serialize};


near_sdk::setup_alloc!();


#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetOptions {
  asset_token_id: String,
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
    pub fn add_asset(&mut self, asset_contract_id: AccountId, asset_token_id: String, extra: String) {
      assert!(
        env::predecessor_account_id() == self.owner_id,
        "Not an owner"
      );

      assert!(
        env::is_valid_account_id(asset_contract_id.as_bytes()),
        "Token Account ID is invalid"
      );

      let options: AssetOptions = AssetOptions {
        asset_token_id: asset_token_id,
        extra: extra,
      };

      self.ingame_assets.insert(&asset_contract_id, &options);
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