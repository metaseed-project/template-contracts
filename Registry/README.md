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

5QW3UfXR34Ron84yC9PUcYMEXG1m9ETAZzBzCPxUWwzynjySnA4usMiMYeHYoeLgasfmkj689ZudmQsMjoLTq4BH
ed25519:3N5GBc4Sa7BcK2CfJzR9rnChq4wtJdFonaeS7ioLckDDKU2iC1Zvtg5sgxi2G3dGrZuwxPVx3qFeRXwR8taUDPx4
