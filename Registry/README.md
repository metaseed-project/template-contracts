# Registry

ID:
ivikkktest.testnet

near-cli login

near create-account registry.ivikkktest.testnet --masterAccount ivikkktest.testnet --initialBalance 5

yarn test
yarn build:contract

near deploy --accountId registry.ivikkktest.testnet --wasmFile out/main.wasm

near view registry.ivikkktest.testnet getLength '{}'
near view registry.ivikkktest.testnet getGameAddress '{"id": 0}'
near view registry.ivikkktest.testnet getGameAddresses '{"id": 0}'

near call registry.ivikkktest.testnet addGameAddress '{"text": "ssasaassa", "name": "name"}' --accountId registry.ivikkktest.testnet
