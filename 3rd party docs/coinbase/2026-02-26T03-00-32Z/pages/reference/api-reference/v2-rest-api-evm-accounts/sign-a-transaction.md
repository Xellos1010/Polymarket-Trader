# sign a transaction

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

A JWT signed using your Wallet Secret, encoded in base64. Refer to the [Generate Wallet Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-wallet-token) section of our Authentication docs for more details on how to generate your Wallet Token.

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Path Parameters

The 0x-prefixed address of the EVM account.

#### Body

The RLP-encoded transaction to sign, as a 0x-prefixed hex string.

Example:

`"0xf86b098505d21dba00830334509431415daf58e2c6b7323b4c58712fd92952145da79018080"`

#### Response

Successfully signed transaction.

The RLP-encoded signed transaction, as a 0x-prefixed hex string.

Example:

`"0x1b0c9cf8cd4554c6c6d9e7311e88f1be075d7f25b418a044f4bf2c0a42a93e212ad0a8b54de9e0b5f7e3812de3f2c6cc79aa8c3e1c02e7ad14b4a8f42012c2c01b"`