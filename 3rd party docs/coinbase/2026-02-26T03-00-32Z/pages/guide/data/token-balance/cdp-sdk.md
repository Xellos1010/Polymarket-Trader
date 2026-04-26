# cdp sdk

## Overview

The CDP SDK allows you to retrieve token balances of an address using the `listTokenBalances` method for Base. For Solana token balances, use the [REST API](https://developer.chrome.com/api-reference/v2/rest-api/solana-token-balances/list-solana-token-balances) directly. Additional information can be found in our [SDK Reference](https://coinbase.github.io/cdp-sdk/typescript/classes/Client.EvmClient.html#listtokenbalances). In this guide, you will learn how to retrieve ERC-20 and native gas token balances of an address on Base networks.

## Prerequisites

-   [Node.js](https://nodejs.org/en/download/) installed
-   A free account logged in on [CDP Portal](https://portal.cdp.coinbase.com/) and a [Secret API key](https://portal.cdp.coinbase.com/projects/api-keys)

### Configure

## Example

In the example below, we query token balances for a known exchange address on Base mainnet.

After running the snippet above, you should see the following output:

```
Checking wallet: 0x835678a611b28684005a5e2233695fb6cbbb0007
Network: base
Token: 150.00 (contract: 0x1198CabDb2b9fF79EC8CbaFfB8977DAF74AFa25a)
Token: 87331987.01 (contract: 0x6D51bC9d512072B6399B81c73F02ba935B2771e5)
Token: 11000.00 (contract: 0x260b9AC75753FbD67F2Ea6D10724dd89a52C1913)
Token: 1261267.05 (contract: 0x3A95F48Cb4c04Eb0EC2a54d72DAA9e1138D9238d)
Token: 2900.00 (contract: 0x1aD2449781a03197BD6A072598Ac311B8bA1f5BD)
Token: 3086.69 (contract: 0xd6e03dEd9Cf9213b207E69570561E08cc3BC681e)

```

This output shows:

-   **Token balances**: Each ERC-20 token with its amount and contract address
-   **Contract addresses**: The long hex strings identify each specific token contract on Base mainnet
-   **Large holdings**: This exchange wallet holds millions of various tokens

## Sample response

The token balance response provides detailed token information:

```
{
  "balances": [
    {
      "token": {
        "network": "base",
        "symbol": "ETH",
        "name": "Ether",
        "contractAddress": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
      },
      "amount": {
        "amount": "1250000000000000000",
        "decimals": 18
      }
    }
  ],
  "nextPageToken": "..."
}

```

### Response fields

Field

Description

`balances`

The list of EVM token balances.

`amount`

Amount of a given token in JSON format, includes `amount`, denominated in the smallest indivisible unit of the token; and `decimals`.

`token`

General information about a token in JSON format.

`network`

Name of the blockchain network, e.g. `base`.

`symbol`

The symbol of the ERC-20 or native gas token, e.g. `ETH`.

`name`

The name of the ERC-20 or native gas token, e.g. `Ether`.

`contractAddress`

Address of the ERC-20 or native gas token smart contract.

`nextPageToken`

The token for the next page of items, if any.

`listTokenBalances` supports paginated responses for addresses with many balances:

## What to read next

-   **[SDK Reference](https://coinbase.github.io/cdp-sdk/typescript/classes/Client.EvmClient.html#listtokenbalances)**: SDK documentation for Base `listTokenBalances`
-   **[Base API Reference](https://developer.chrome.com/api-reference/v2/rest-api/evm-token-balances/list-evm-token-balances)**: Base REST API endpoint details
-   **[Solana API Reference](https://developer.chrome.com/api-reference/v2/rest-api/solana-token-balances/list-solana-token-balances)**: Solana REST API endpoint details