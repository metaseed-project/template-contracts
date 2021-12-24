use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct GameManager {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    val: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

#[near_bindgen]
impl GameManager {
    /// Returns 8-bit signed integer of the game_manager value.
    ///
    /// This must match the type from our struct's 'val' defined above.
    ///
    /// Note, the parameter is `&self` (without being mutable) meaning it doesn't modify state.
    /// In the frontend (/src/main.js) this is added to the "viewMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near view game_manager.YOU.testnet get_num
    /// ```
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    /// Increment the game_manager.
    ///
    /// Note, the parameter is "&mut self" as this function modifies state.
    /// In the frontend (/src/main.js) this is added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call game_manager.YOU.testnet increment --accountId donation.YOU.testnet
    /// ```
    pub fn increment(&mut self) {
        // note: adding one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i8::wrapping_add(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_add
        self.val += 1;
        let log_message = format!("Increased number to {}", self.val);
        env::log(log_message.as_bytes());
        after_game_manager_change();
    }

    /// Decrement (subtract from) the game_manager.
    ///
    /// In (/src/main.js) this is also added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call game_manager.YOU.testnet decrement --accountId donation.YOU.testnet
    /// ```
    pub fn decrement(&mut self) {
        // note: subtracting one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i8::wrapping_sub(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_sub
        self.val -= 1;
        let log_message = format!("Decreased number to {}", self.val);
        env::log(log_message.as_bytes());
        after_game_manager_change();
    }

    /// Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        // Another way to log is to cast a string into bytes, hence "b" below:
        env::log(b"Reset game_manager to zero");
    }
}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
fn after_game_manager_change() {
    // show helpful warning that i8 (8-bit signed integer) will overflow above 127 or below -128
    env::log("Make sure you don't overflow, my friend.".as_bytes());
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-game_manager-tutorial -- --nocapture
 * Note: 'rust-game_manager-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
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

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the game_manager at zero
        let mut contract = GameManager { val: 0 };
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        // confirm that we received 1 when calling get_num
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = GameManager { val: 0 };
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = GameManager { val: 0 };
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(0, contract.get_num());
    }
}