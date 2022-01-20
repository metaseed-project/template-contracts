# deploy Registry => minting NFT => provide NFT to Registry

#### helpers

near create-account registry4.$Admin --masterAccount $Admin

## actors

Admin=metaseed.testnet

ContractId=registry4.$Admin

Actor=phoneiostest.testnet

## deploy

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/registry.wasm

### mint

NFTC=example-nft.testnet
tokenId=1111

near call $NFTC nft_mint '{"token_id": "'$tokenId'", "receiver_id": "'$Actor'", "token_metadata": { "title": "test", "description": "test", "media": "https://bafybeidl4hjbpdr6u6xvlrizwxbrfcyqurzvcnn5xoilmcqbxfbdwrmp5m.ipfs.dweb.link/", "copies": 1}}' --accountId $Actor --deposit 0.1

near view $NFTC nft_tokens_for_owner '{"account_id": "'$Actor'"}'

## Add

near call $NFTC nft_transfer_call '{"receiver_id": "'$ContractId'", "token_id": "'$tokenId'", "msg": ""}' --accountId $Actor --depositYocto 1 --gas 300000000000000

### check

near view example-nft.testnet nft_tokens_for_owner '{"account_id": "'$ContractId'"}'

near view example-nft.testnet nft_tokens_for_owner '{"account_id": "'$Actor'"}'
