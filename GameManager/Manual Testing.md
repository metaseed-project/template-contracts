### This will become automated

ContractID=dev-1640363567306-23369340129852

Creator=phoneiostest.testnet

Receiver=ivikkktest.testnet

near call $ContractID create_ingame_nft '{"prefix": "nft3"}' --accountId $Creator --depositYocto 5189980000000000000000000 --gas 300000000000000

near call $ContractID get_asset '{"account_id": "nft3.$ContractID"}' --accountId $Creator

## will work

near call nft3.$ContractID nft_mint '{"token_id": "1", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $Creator --deposit 0.1

## won't

near call nft3.$ContractID nft_mint '{"token_id": "0", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $ContractID --deposit 0.1

near call nft3.$ContractID nft_mint '{"token_id": "1", "receiver_id": "'$Receiver'", "token_metadata": { "title": "t", "description": "d", "media": "m", "copies": 1}}' --accountId $Receiver --deposit 0.1
