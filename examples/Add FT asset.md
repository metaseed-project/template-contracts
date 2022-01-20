# deploy Registry => minting FT => provide FT to Registry

#### helpers

near create-account registry4.$Admin --masterAccount $Admin

## actors

Admin=metaseed.testnet

ContractId=dev-1642684638853-24488548109533

Actor=phoneiostest.testnet

## deploy

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/registry.wasm

near call $ContractId new '{"owner_id": "'$Admin'"}' --accountId $Admin

#### mint some tokens for account

near call ft.examples.testnet ft_mint '{"receiver_id": "'$ContractId'", "amount": "1"}' --deposit 0.1 --accountId $ContractId

near view ft.examples.testnet ft_balance_of '{"account_id": "'$ContractId'"}'

### mint

FTC=ft.examples.testnet

near call ft.examples.testnet ft_mint '{"receiver_id": "'$Actor'", "amount": "25"}' --deposit 0.1 --accountId $Actor

near view ft.examples.testnet ft_balance_of '{"account_id": "'$Actor'"}'

## Add

near call $FTC ft_transfer_call '{"receiver_id": "'$ContractId'", "amount": "2", "msg":"{\"receiver_id\": \"'$Actor'\"}" }' --accountId $Actor --depositYocto 1 --gas 300000000000000

near call $FTC ft_transfer_call '{"receiver_id": "'$ContractId'", "amount": "2", "msg": "" }' --accountId $Actor --depositYocto 1 --gas 300000000000000

### check

near view ft.examples.testnet ft_balance_of '{"account_id": "'$ContractId'"}'

near view ft.examples.testnet ft_balance_of '{"account_id": "'$Actor'"}'

near view $ContractId get_ballances '{"from_index": 0, "limit": 10}'
