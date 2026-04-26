# create a spend permission

Create a spend permission

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

A JWT signed using your Wallet Secret, encoded in base64. Refer to the [Generate Wallet Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-wallet-token) section of our Authentication docs for more details on how to generate your Wallet Token.

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Path Parameters

The address of the Smart Account to create the spend permission for.

#### Body

Request parameters for creating a Spend Permission.

The network the spend permission is on.

Available options

:

`base`,

`base-sepolia`,

`ethereum`,

`ethereum-sepolia`,

`optimism`,

`arbitrum`,

`avalanche`,

`polygon`

Entity that can spend account's tokens. Can be either a Smart Account or an EOA.

Example:

`"0x9Fb909eA400c2b8D99Be292DADf07e63B814527c"`

ERC-7528 native token address (e.g. "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE" for native ETH), or an ERC-20 contract address.

Example:

`"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"`

Maximum allowed value to spend, in atomic units for the specified token, within each period.

Example:

`"1000000000000000000"`

Time duration for resetting used allowance on a recurring basis (seconds).

The start time for this spend permission, in Unix seconds.

The expiration time for this spend permission, in Unix seconds.

Example:

`"281474976710655"`

An arbitrary salt to differentiate unique spend permissions with otherwise identical data.

Example:

`"95959551014433038874972658238091428449162862973207257628575040053304171156143"`

Arbitrary data to include in the permission.

The paymaster URL of the spend permission.

Required string length: `11 - 2048`

Example:

`"https://paymaster.cdp.coinbase.com"`

#### Response

Successfully created spend permission.

The network the user operation is for.

Available options

:

`base-sepolia`,

`base`,

`arbitrum`,

`optimism`,

`zora`,

`polygon`,

`bnb`,

`avalanche`,

`ethereum`,

`ethereum-sepolia`

The hash of the user operation. This is not the transaction hash, as a transaction consists of multiple user operations. The user operation hash is the hash of this particular user operation which gets signed by the owner of the Smart Account.

Example:

`"0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"`

The list of calls in the user operation.

Example:

```
[  
  {  
    "to": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",  
    "value": "0",  
    "data": "0xa9059cbb000000000000000000000000fc807d1be4997e5c7b33e4d8d57e60c5b0f02b1a0000000000000000000000000000000000000000000000000000000000000064"  
  },  
  {  
    "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",  
    "value": "1000000000000000",  
    "data": "0x"  
  }  
]
```

The status of the user operation.

Available options

:

`pending`,

`signed`,

`broadcast`,

`complete`,

`dropped`,

`failed`

The hash of the transaction that included this particular user operation. This gets set after the user operation is broadcasted and the transaction is included in a block.

Example:

`"0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"`

The list of receipts associated with the user operation.

Example:

```
[  
  {  
    "revert": {  
      "data": "0x123",  
      "message": "reason for failure"  
    },  
    "blockHash": "0x386544b58930c0ec9e8f3ed09fb4cdb76b9ae0a1a37ddcacebe3925b57978e65",  
    "blockNumber": 29338819,  
    "gasUsed": "100000"  
  }  
]
```