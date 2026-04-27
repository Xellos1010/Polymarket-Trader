# send a transaction

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

A JWT signed using your Wallet Secret, encoded in base64. Refer to the [Generate Wallet Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-wallet-token) section of our Authentication docs for more details on how to generate your Wallet Token.

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Path Parameters

The 0x-prefixed address of the Ethereum account.

#### Body

The network to send the transaction to.

Available options

:

`base`,

`base-sepolia`,

`ethereum`,

`ethereum-sepolia`,

`avalanche`,

`polygon`,

`optimism`,

`arbitrum`

The RLP-encoded transaction to sign and send, as a 0x-prefixed hex string.

Example:

`"0xf86b098505d21dba00830334509431415daf58e2c6b7323b4c58712fd92952145da79018080"`

#### Response

Successfully signed and sent transaction.

The hash of the transaction, as a 0x-prefixed hex string.

Example:

`"0xf8f98fb6726fc936f24b2007df5cb20e2b8444ff3dfaa2a929335f432a9be2e7"`