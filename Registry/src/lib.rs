use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, Gas, PromiseOrValue};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json;


near_sdk::setup_alloc!();

const DEPLOY_ATTACHED_BALANCE: Balance = 0;

const NFT_TRANSFER_GAS: Gas = 30_000_000_000_000;

const FT_TRANSFER_GAS: Gas = 30_000_000_000_000;

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

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, PartialEq, Eq)]
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

#[derive(BorshDeserialize, BorshSerialize, Deserialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OnTransferArgs {
  receiver_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTTransferArgs {
  receiver_id: AccountId,
  token_id: String,
  memo: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FTTransferArgs {
  receiver_id: AccountId,
  amount: String,
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

    pub fn transfer_asset(&mut self, asset_id: String, receiver_id: String, amount: u128) {
      assert!(
        self.ballances.get(&asset_id).is_some(),
        "Asset not exist"
      );

      let sender = env::predecessor_account_id();

      let asset = self.ballances.get(&asset_id).unwrap();
      assert!(
        asset.owner == sender,
        "You are not an owner"
      );

      if asset.asset_type == AssetType::NFT {
        let asset: Asset = Asset {
          asset_type: AssetType::NFT,
          owner: receiver_id,
          amount: 1,
        };
        self.ballances.insert(&asset_id, &asset);

      } else if asset.asset_type == AssetType::FT {
        assert!(
          asset.amount >= amount,
          "Amount is not enough"
        );

        let asset_sender: Asset = Asset {
          asset_type: AssetType::FT,
          owner: asset.owner,
          amount: asset.amount - amount,
        };
        self.ballances.insert(&asset_id, &asset_sender);

        
        let split: Vec<&str> = asset_id.split(":").collect();
        let ft_contract_id: String = split[1].to_string();
        let receiver_and_token_id = format!("{}:{}", receiver_id, ft_contract_id);

        let mut transfered_amount = amount;
        if self.ballances.get(&receiver_and_token_id).is_some() {
          let ballance = self.ballances.get(&receiver_and_token_id).unwrap();
          transfered_amount += ballance.amount;
        }
  
        let asset_receiver: Asset = Asset {
          asset_type: AssetType::FT,
          owner: receiver_id,
          amount: transfered_amount,
        };
  
        self.ballances.insert(&receiver_and_token_id, &asset_receiver);
      }

    }

    #[payable]
    pub fn withdraw_nft(&mut self, asset_id: String, receiver_id: String) -> Promise {
      assert!(
        self.ballances.get(&asset_id).is_some(),
        "Asset not exist"
      );
      let asset = self.ballances.get(&asset_id).unwrap();
      assert!(
        asset.owner == env::predecessor_account_id(),
        "You are not an owner"
      );

      let split: Vec<&str> = asset_id.split(":").collect();
      let nft_contract_id: String = split[0].to_string();
      let tokne_id: String = split[1].to_string();

      let args: NFTTransferArgs = NFTTransferArgs {
        receiver_id: receiver_id,
        token_id: tokne_id,
        memo: "Transfer from registry".to_string(),
      };

      self.ballances.remove(&asset_id);

      Promise::new(nft_contract_id)
        .function_call(
          b"nft_transfer".to_vec(),
          serde_json::to_vec(&args).unwrap(),
          env::attached_deposit(),
          NFT_TRANSFER_GAS
        )
    }

    #[payable]
    pub fn withdraw_ft(&mut self, asset_id: String, receiver_id: String, amount: u128) -> Promise {
      assert!(
        self.ballances.get(&asset_id).is_some(),
        "Asset not exist"
      );
      let asset = self.ballances.get(&asset_id).unwrap();
      assert!(
        asset.owner == env::predecessor_account_id(),
        "You are not an owner"
      );

      assert!(
        asset.amount >= amount,
        "Amount is not enough"
      );

      let split: Vec<&str> = asset_id.split(":").collect();
      let ft_contract_id: String = split[1].to_string();

      let asset_after: Asset = Asset {
        asset_type: AssetType::FT,
        owner: asset.owner,
        amount: asset.amount - amount,
      };

      self.ballances.insert(&asset_id, &asset_after);

      let args: FTTransferArgs = FTTransferArgs {
        receiver_id: receiver_id,
        amount: amount.to_string(),
      };

      Promise::new(ft_contract_id)
        .function_call(
          b"ft_transfer".to_vec(),
          serde_json::to_vec(&args).unwrap(),
          env::attached_deposit(),
          FT_TRANSFER_GAS
        )
    }

    //-- add_nft_asset
    pub fn nft_on_transfer(&mut self, sender_id: AccountId, token_id: String, msg: String) -> PromiseOrValue<bool> {
      let nft_contract_id = env::predecessor_account_id();
      let mut owner = sender_id;

      if !msg.is_empty() {
        let OnTransferArgs {
            receiver_id,
        } = near_sdk::serde_json::from_str(&msg).expect("Invalid OnTransferArgs");

        if env::is_valid_account_id(receiver_id.as_bytes()) {
          owner = receiver_id;
        }
      }

      let contract_and_token_id = format!("{}:{}", nft_contract_id, token_id);

      assert!(
        self.ballances.get(&contract_and_token_id).is_none(),
        "Already exist"
      );

      let asset: Asset = Asset {
        asset_type: AssetType::NFT,
        owner: owner,
        amount: 1,
      };

      self.ballances.insert(&contract_and_token_id, &asset);
      PromiseOrValue::Value(false)
    }

    //-- add_ft_asset
    pub fn ft_on_transfer(&mut self, sender_id: AccountId, amount: String, msg: String) -> PromiseOrValue<U128> {
      let ft_contract_id = env::predecessor_account_id();
      let mut owner = sender_id;

      if !msg.is_empty() {
        let OnTransferArgs {
            receiver_id,
        } = near_sdk::serde_json::from_str(&msg).expect("Invalid OnTransferArgs");

        if env::is_valid_account_id(receiver_id.as_bytes()) {
          owner = receiver_id;
        }
      }

      let user_and_token_id = format!("{}:{}", owner, ft_contract_id);

      let mut transfered_amount = amount.parse().unwrap();
      if self.ballances.get(&user_and_token_id).is_some() {
        let ballance = self.ballances.get(&user_and_token_id).unwrap();
        transfered_amount += ballance.amount;
      }

      let asset: Asset = Asset {
        asset_type: AssetType::FT,
        owner: owner,
        amount: transfered_amount,
      };

      self.ballances.insert(&user_and_token_id, &asset);

      PromiseOrValue::Value(U128(0))
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

    pub fn get_games(&self, from_index: u64, limit: u64) -> Vec<(AccountId, GameOptions)> {
      let keys = self.game_contracts.keys_as_vector();
      let values = self.game_contracts.values_as_vector();
      (from_index..std::cmp::min(from_index + limit, self.game_contracts.len()))
          .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap()))
          .collect()
    }

    pub fn get_ballance(&self, ballance_address: AccountId) -> Option<Asset> {
      return self.ballances.get(&ballance_address);
    }
    
    pub fn get_balances_counts(&self) -> u64 {
        return self.ballances.len();
    }
    
    pub fn get_ballances(&self, from_index: u64, limit: u64) -> Vec<(AccountId, Asset)> {
      let keys = self.ballances.keys_as_vector();
      let values = self.ballances.values_as_vector();
      (from_index..std::cmp::min(from_index + limit, self.ballances.len()))
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