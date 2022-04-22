# deploy Registry => minting FT => provide FT to Registry

#### helpers

near create-account registry4.$Admin --masterAccount $Admin

## actors

Admin=metaseed.testnet

ContractId=dev-1649598123897-66232236184396

Actor=phoneiostest.testnet
Actor=123321_2.testnet

## deploy

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

near dev-deploy --wasmFile res/fungible_token.wasm  --accountId $Admin

near call $ContractId new '{"owner_id": "'$Admin'", "total_supply": "1000", "metadata": { "spec": "ft-1.0.0", "name": "NEAR Space Key", "symbol": "NSK", "icon": "https://nahbnow.com/wp-content/uploads/2016/08/key_orange.jpg", "decimals": 0 }}' --accountId $Admin

near view $ContractId ft_metadata

near view $ContractId ft_balance_of '{"account_id": "'$Admin'"}'

near call $ContractId storage_deposit '{"account_id": "'$Actor'", "registration_only": true}' --accountId $Admin --amount 0.00125

near view $ContractId ft_balance_of '{"account_id": "'$Actor'"}'

near view $ContractId ft_balance_of '{"account_id": "'$Admin'"}'

near call $ContractId ft_transfer '{"receiver_id": "'$Actor'", "amount": "1" }' --accountId $Admin --depositYocto 1 --gas 300000000000000

near call $ContractId ft_transfer '{"receiver_id": "'$Actor'", "amount": "0.1" }' --accountId $Admin --depositYocto 1













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



near view ft.examples.testnet ft_balance_of '{"account_id": "'$Actor'"}'

near view $ContractId get_ballances '{"from_index": 0, "limit": 10}'
