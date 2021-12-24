# Non-fungible Token (NFT)

# Building this contract

```bash
./build.sh
```

# Testing this contract

```bash
cargo test -- --nocapture
```

### Quickest deploy

```bash
near dev-deploy --wasmFile res/non_fungible_token.wasm
```

```bash
source neardev/dev-account.env
```

```bash
echo $CONTRACT_NAME
```

```bash
near call $CONTRACT_NAME new_default_meta '{"owner_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME
```

```bash
near view $CONTRACT_NAME nft_metadata
```

### Standard deploy

near login

ID=MY_ACCOUNT_NAME

echo $ID

near deploy --wasmFile res/non_fungible_token.wasm --accountId $ID

near call $ID new_default_meta '{"owner_id": "'$ID'"}' --accountId $ID

We'll be able to view our metadata right after:

near view $ID nft_metadata

### Mint

near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "token_metadata": { "title": "1", "description": "2", "media": "3", "copies": 1}}' --accountId $ID --deposit 0.1

# Transferring our NFT

Let's set up an account to transfer our freshly minted token to. This account will be a sub-account of the NEAR account we logged in with originally via `near login`.

    near create-account alice.$ID --masterAccount $ID --initialBalance 10

Checking Alice's account for tokens:

    near view $ID nft_tokens_for_owner '{"account_id": "'alice.$ID'"}'

Then we'll transfer over the NFT into Alice's account. Exactly 1 yoctoNEAR of deposit should be attached:

    near call $ID nft_transfer '{"token_id": "0", "receiver_id": "alice.'$ID'", "memo": "transfer ownership"}' --accountId $ID --depositYocto 1

Checking Alice's account again shows us that she has the Olympus Mons token.
