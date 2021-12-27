# This will become automated

## actors

near create-account registry2.$Admin --masterAccount $Admin

Admin=metaseed.testnet

ContractId=registry2.$Admin

GD=phoneiostest.testnet

## deploy

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

near deploy --wasmFile target/wasm32-unknown-unknown/release/registry.wasm --accountId $ContractId

### initialized

near call $ContractId new '{"owner_id": "'$Admin'"}' --accountId $Admin

## use

### facory

GAME_NAME=game1

near call $ContractId create_game_manager '{"prefix": "'$GAME_NAME'"}' --accountId $GD --depositYocto 5789980000000000000000000 --gas 300000000000000

### factory2

NFT_PREFIX=nft4

near call $GAME_NAME.$ContractID create_ingame_nft '{"prefix": "'$NFT_PREFIX'"}' --accountId $GD --depositYocto 5189980000000000000000000 --gas 300000000000000
