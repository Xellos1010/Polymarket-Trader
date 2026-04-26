# addresses

An address is a user-controlled account on a blockchain network. An address can hold a balance of one or more assets, and can be used to send or receive crypto. An address is backed by a [private key](https://help.coinbase.com/en/coinbase/getting-started/crypto-education/glossary/private-key), which provides access to spend the assets in the address. Addresses can list their balances and support various verbs depending on the type of address.

## Address Types

Depending on how the associated private key is managed, addresses can be classified into two types, [wallet](#wallet-addresses) and [external](#external-addresses).

### Wallet Addresses

Wallet addresses belong to developer custodied wallets. See [Creating a Wallet](https://developer.chrome.com/server-wallets/v1/concepts/wallets#creating-a-wallet) section. The private keys for these addresses are managed by the CDP SDK. Wallet addresses support all verbs that CDP SDK supports.

### External Addresses

External addresses do not belong to CDP Wallets. The CDP SDK can be used to interact with these addresses; but because the SDK does not manage private keys for them, all signing operations are done off-platform. External addresses only support [retrieving balances](https://developer.chrome.com/server-wallets/v1/concepts/wallets#retrieving-balances), staking verbs, and funding with testnet tokens with the [faucet](https://developer.chrome.com/server-wallets/v1/concepts/wallets).

-   Typescript
    
-   Python
    

## Creating Wallet Addresses

Wallet addresses belong to a [wallet](https://developer.chrome.com/server-wallets/v1/concepts/wallets):

A wallet comes with a single address by default, accessible via `default_address`.

## Creating External Addresses

To create an External Address object, provide the address string and the network:

## Viewing Address IDs

To view the hexadecimal string that actually represents your address, use the `address_id` property:

## Listing Address historical balances for an asset

To view the historical balances of [assets](https://developer.chrome.com/server-wallets/v1/concepts/assets) of an address, call the following:

## Listing Address transactions

To view all transactions for a specific address in the blockchain, you can use the following code snippet: