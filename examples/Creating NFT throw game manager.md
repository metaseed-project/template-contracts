# Creatin NFT throw Game manager

## actors

Admin=ivikkktest.testnet

ContractID=gamemanager.$Admin

Creator=phoneiostest.testnet

Receiver=testplayer.testnet

near create-account gamemanager.$Admin --masterAccount $Admin

## deploy

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

near deploy --wasmFile target/wasm32-unknown-unknown/release/game_manager.wasm --accountId $ContractID

### initialized

near call $ContractID new '{"owner_id": "'$Creator'"}' --accountId $Creator

## use

### facory

NFT_PREFIX=nft4

near call $ContractID create_ingame_nft '{"prefix": "'$NFT_PREFIX'"}' --accountId $Creator --depositYocto 5189980000000000000000000 --gas 300000000000000

### get

near view $ContractID get_asset '{"account_id": "'$NFT_PREFIX'.'$ContractID'"}' --accountId $Creator

### set

near call $ContractID set_asset '{"account_id": "'$NFT_PREFIX'.'$ContractID'", "extra": ""}' --accountId $Creator

## will work

near call $NFT_PREFIX.$ContractID nft_mint '{"token_id": "1", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $Creator --deposit 0.1

## won't

near call nft3.$ContractID nft_mint '{"token_id": "0", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $ContractID --deposit 0.1

near call nft3.$ContractID nft_mint '{"token_id": "1", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $Receiver --deposit 0.1
