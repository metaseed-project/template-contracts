use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, Gas};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json;


near_sdk::setup_alloc!();

const DEPLOY_ATTACHED_BALANCE: Balance = 0;

const GM_GAS_NEW: Gas = 50_000_000_000_000;

const GM_WASM_CODE: &[u8] = include_bytes!("../../GameManager/target/wasm32-unknown-unknown/release/game_manager.wasm");

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GMArgs {
    pub owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameOptions {
  extra: String,
  publisher_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum AssetType {
  NFT,
  FT,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Asset {
  asset_type: AssetType,
  owner: AccountId,
  amount: u128,
}

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Registry {
    pub owner_id: AccountId,
    pub game_contracts: UnorderedMap<AccountId, GameOptions>,
    pub ballances: UnorderedMap<String, Asset>,
}



#[near_bindgen]
impl Registry {

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
          owner_id,
          game_contracts: UnorderedMap::new(b"r".to_vec()),
          ballances: UnorderedMap::new(b"r".to_vec()),
        }
    }

    pub fn add_nft_asset(&mut self, nft_contract_id: AccountId, token_id: String) {
      let contract_and_token_id = format!("{}:{}", nft_contract_id, token_id);

      assert!(
        self.ballances.get(&contract_and_token_id).is_none(),
        "Already exist"
      );

      //Check if NFT provided

      let asset: Asset = Asset {
        asset_type: AssetType::NFT,
        owner: env::predecessor_account_id(),
        amount: 1,
      };

      self.ballances.insert(&contract_and_token_id, &asset);
    }

    pub fn add_ft_asset(&mut self, ft_contract_id: AccountId, amount: u128) {
      let user_and_token_id = format!("{}:{}", env::predecessor_account_id(), ft_contract_id);

      let mut initial_amount = 0;
      if self.ballances.get(&user_and_token_id).is_some() {
        let ballance = self.ballances.get(&user_and_token_id).unwrap();
        initial_amount += ballance.amount;
      }

      //Check if FT provided

      let asset: Asset = Asset {
        asset_type: AssetType::FT,
        owner: env::predecessor_account_id(),
        amount: initial_amount + amount,
      };

      self.ballances.insert(&user_and_token_id, &asset);
    }

    #[payable]
    pub fn create_game_manager(&mut self, prefix: AccountId) {
        let subaccount_id = create_gm_subaccount(prefix);

        assert!(
          self.game_contracts.get(&subaccount_id).is_none(),
          "Already exist"
        );

        let options: GameOptions = GameOptions {
          extra: "".to_string(),
          publisher_id: env::predecessor_account_id(),
        };

        self.game_contracts.insert(&subaccount_id, &options);

        create_gm_contract(subaccount_id,  GM_WASM_CODE.to_vec());
    }

    pub fn get_game(&self, game_address: AccountId) -> Option<GameOptions> {
        return self.game_contracts.get(&game_address);
    }

    pub fn get_counts(&self) -> u64 {
        return self.game_contracts.len();
    }

    /// Retrieves multiple elements from the `game_contracts`.
    pub fn get_games(&self, from_index: u64, limit: u64) -> Vec<(AccountId, GameOptions)> {
      let keys = self.game_contracts.keys_as_vector();
      let values = self.game_contracts.values_as_vector();
      (from_index..std::cmp::min(from_index + limit, self.game_contracts.len()))
          .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap()))
          .collect()
    }
}

fn create_gm_subaccount(prefix: AccountId) -> String {
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

fn create_gm_contract(subaccount_id: AccountId, code: Vec<u8>) -> Promise {
    assert!(
      env::attached_deposit() >= DEPLOY_ATTACHED_BALANCE,
      "Not enough attached deposit"
    );

    let args: GMArgs = GMArgs {
      owner_id: env::predecessor_account_id(),
    };

    Promise::new(subaccount_id)
        .create_account()
        .transfer(env::attached_deposit())
        .add_full_access_key(env::signer_account_pk())
        .deploy_contract(code)
        .function_call(
          b"new".to_vec(),
          serde_json::to_vec(&args).unwrap(),
          0,
          GM_GAS_NEW
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