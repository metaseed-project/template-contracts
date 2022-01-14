# Creating GC => NFT contract => minting NFT

## actors

near create-account registry4.$Admin --masterAccount $Admin

Admin=metaseed.testnet

ContractId=registry4.$Admin

GD=phoneiostest.testnet

Receiver=testplayer.testnet

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

near call $GAME_NAME.$ContractId create_ingame_nft '{"prefix": "'$NFT_PREFIX'"}' --accountId $GD --depositYocto 5189980000000000000000000 --gas 300000000000000

### get

near view $GAME_NAME.$ContractId get_asset '{"account_id": "'$NFT_PREFIX.$GAME_NAME.$ContractId'"}' --accountId $GD

### Mint NFT

near call $NFT_PREFIX.$GAME_NAME.$ContractId nft_mint '{"token_id": "1", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $GD --deposit 0.1

### Browse registry

near view $ContractId get_counts {} --accountId $GD

near view $ContractId get_games '{"from_index": 0, "limit": 10}' --accountId $GD

### Browse GC

near view $GAME_NAME.$ContractId get_counts {} --accountId $GD

near view $GAME_NAME.$ContractId get_assets '{"from_index": 0, "limit": 10}' --accountId $GD

### Change GC

near call $GAME_NAME.$ContractId set_asset '{"account_id": "nft4.game1.registry4.metaseed.testnet", "extra": "{'force': 10}"}' --accountId $GD --deposit 1
