### Create Game contract

ContractId=registry4.metaseed.testnet

GD=game_creator1.testnet

GAME_NAME=my_game_name

near call $ContractId create_game_manager '{"prefix": "'$GAME_NAME'"}' --accountId $GD --depositYocto 5789980000000000000000000 --gas 300000000000000

### Create an NFT

NFT_PREFIX=super_sword_collection

near call $GAME_NAME.$ContractId create_ingame_nft '{"prefix": "'$NFT_PREFIX'"}' --accountId $GD --depositYocto 5189980000000000000000000 --gas 300000000000000

### Check data

near view $GAME_NAME.$ContractId get_asset '{"account_id": "'$NFT_PREFIX.$GAME_NAME.$ContractId'"}' --accountId $GD

### Mint Specific sword

Receiver=phoneiostest.testnet

echo $NFT_PREFIX.$GAME_NAME.$ContractId

near call $NFT_PREFIX.$GAME_NAME.$ContractId nft_mint '{"token_id": "2", "receiver_id": "'$Receiver'", "token_metadata": { "title": "sword1", "description": "Super rare sword", "media": "https://bafkreifeih3hr6g5e3alrngb3rsrkvb7wwspibwreu6wagwwfietbboa6u.ipfs.dweb.link", "copies": 1}}' --accountId $GD --deposit 0.1
