Admin=nearspacecontract.testnet

Fuel=fuel.$Admin
Artifacts=artifacts.$Admin
Planets=planets_v1.$Admin

near create-account $Fuel --masterAccount $Admin
near create-account $Artifacts --masterAccount $Admin
near create-account $Planets --masterAccount $Admin

### Planets

near deploy --wasmFile ./SimpleNFT/res/non_fungible_token.wasm --accountId $Planets

near call $Planets new '{"owner_id": "phoneiostest.testnet", "metadata": { "name": "Near Space Planets", "symbol": "NSP", "icon": null, "base_uri": null, "reference": null, "reference_hash": null, "spec": "nft-1.0.0" }}' --accountId $Planets

near view $Planets nft_metadata
near call $Planets nft_mint '{"token_id": "0", "receiver_id": "'$Admin'", "token_metadata": { "title": "Test Planet", "description": "", "media": "", "copies": 1}}' --accountId phoneiostest.testnet --deposit 0.1

### Fuel

near deploy --wasmFile SimpleFT/res/fungible_token.wasm --accountId $Fuel

near call $Fuel new '{"owner_id": "'$Admin'", "total_supply": "34028237000000000000000000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Fuel", "symbol": "NEF", "decimals": 8 }}' --accountId $Fuel

### Artifacts

near deploy --wasmFile SimpleFT/res/fungible_token.wasm --accountId $Artifacts

near call $Artifacts new '{"owner_id": "'$Admin'", "total_supply": "34028237000000000000000000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Artifacts", "symbol": "NEA", "decimals": 8 }}' --accountId $Artifacts
