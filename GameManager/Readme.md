### test

cargo test -- --nocapture

### compile

cargo build --target wasm32-unknown-unknown --release

### deploy

ID=YOUR_ACCOUNT

near deploy --wasmFile target/wasm32-unknown-unknown/release/rust_counter_tutorial.wasm --accountId dev-1640363567306-2336934012985

near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/game_manager.wasm --accountId $ID

### helpers

ls ~/.near-credentials/testnet

dev-1640363567306-23369340129852

ivikkktest.testnet

near view dev-1640363567306-23369340129852 get_num '{}' --accountId ivikkktest.testnet
