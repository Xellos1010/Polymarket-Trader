# revoke a spend permission

Revoke a spend permission

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

A JWT signed using your Wallet Secret, encoded in base64. Refer to the [Generate Wallet Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-wallet-token) section of our Authentication docs for more details on how to generate your Wallet Token.

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Path Parameters

The address of the Smart account this spend permission is valid for.

#### Body

Request parameters for revoking a Spend Permission.

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

The hash of the spend permission to revoke.

Example:

`"0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"`

The paymaster URL of the spend permission.

Required string length: `11 - 2048`

Example:

`"https://paymaster.cdp.coinbase.com"`

#### Response

Successfully revoked spend permission.

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