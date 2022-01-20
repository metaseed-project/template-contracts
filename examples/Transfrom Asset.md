#### helpers

near create-account registry4.$Admin --masterAccount $Admin

## actors

Admin=metaseed.testnet

ContractId=dev-1642684638853-24488548109533

Actor=phoneiostest.testnet

Actor2=pety.testnet

## deploy

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/registry.wasm

near call $ContractId new '{"owner_id": "'$Admin'"}' --accountId $Admin

### transform

near call $ContractId transfer_asset '{"asset_id": "example-nft.testnet:1116", "receiver_id": "'$Actor2'", "amount": 1}' --accountId $Actor

near call $ContractId transfer_asset '{"asset_id": "phoneiostest.testnet:ft.examples.testnet", "receiver_id": "'$Actor2'", "amount": 1}' --accountId $Actor

### check

near view $ContractId get_ballances '{"from_index": 0, "limit": 10}'
