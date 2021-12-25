### test

cargo test -- --nocapture

### compile

cargo build --target wasm32-unknown-unknown --release

### deploy

ID=dev-1640363567306-23369340129852

near deploy --wasmFile target/wasm32-unknown-unknown/release/rust_counter_tutorial.wasm --accountId $ID

near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/game_manager.wasm

### run

ls ~/.near-credentials/testnet

ivikkktest.testnet

## Create NFT Ingame Contract

near call $ID create_ingame_nft '{"prefix": "magic_sword4"}' --accountId phoneiostest.testnet --depositYocto 3189980000000000000000000

## Call NFT Ingame Contract

near call magic_sword.$ID new_default_meta '{"owner_id": "phoneiostest.testnet"}' --accountId phoneiostest.testnet

near view magic_sword4.$ID nft_metadata

30000000000000
2428023852964
