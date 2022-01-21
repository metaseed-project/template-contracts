#### helpers

near create-account registry4.$Admin --masterAccount $Admin

## actors

Admin=metaseed.testnet

ContractId=dev-1642684638853-24488548109533

Actor=phoneiostest.testnet

Actor2=$Admin

## deploy

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/registry.wasm

near call $ContractId new '{"owner_id": "'$Admin'"}' --accountId $Admin

### withdraw

<!-- this should NOT work -->

near call $ContractId withdraw_ft '{"asset_id": "metaseed.testnet:ft.examples.testnet", "receiver_id": "'$Actor2'", "amount": 2}' --accountId $Actor --gas 300000000000000

<!-- this should work -->

near call $ContractId withdraw_ft '{"asset_id": "metaseed.testnet:ft.examples.testnet", "receiver_id": "'$Actor2'", "amount": 2}' --accountId $Actor2 --gas 300000000000000 --depositYocto 1

near call $ContractId withdraw_nft '{"asset_id": "example-nft.testnet:1125", "receiver_id": "'$Actor'"}' --accountId $Actor --gas 300000000000000 --depositYocto 1

### check

near view $ContractId get_ballances '{"from_index": 0, "limit": 10}'

near view ft.examples.testnet ft_balance_of '{"account_id": "'$Actor2'"}'

near view example-nft.testnet nft_tokens_for_owner '{"account_id": "'$Actor'"}'
