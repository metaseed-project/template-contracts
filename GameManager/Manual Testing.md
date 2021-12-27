# This will become automated

## actors

Admin=ivikkktest.testnet

ContractID=gamemanager.$Admin

Creator=phoneiostest.testnet

Receiver=testplayer.testnet

near create-account gamemanager.$Admin --masterAccount $Admin

## deploy

near deploy --wasmFile target/wasm32-unknown-unknown/release/game_manager.wasm --accountId $ContractID

### initialized

near call $ContractID new '{"owner_id": "'$Creator'"}' --accountId $Creator

## use

### facory

near call $ContractID create_ingame_nft '{"prefix": "nft4"}' --accountId $Creator --depositYocto 5189980000000000000000000 --gas 300000000000000

### get

near view $ContractID get_asset '{"account_id": "nft3.'$ContractID'"}' --accountId $Creator

### set

near call $ContractID set_asset '{"account_id": "nft3.'$ContractID'", "extra": ""}' --accountId $Creator

## will work

near call nft3.$ContractID nft_mint '{"token_id": "1", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $Creator --deposit 0.1

## won't

near call nft3.$ContractID nft_mint '{"token_id": "0", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $ContractID --deposit 0.1

near call nft3.$ContractID nft_mint '{"token_id": "1", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $Receiver --deposit 0.1
