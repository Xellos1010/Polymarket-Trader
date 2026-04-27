# sign a message

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

A JWT signed using your Wallet Secret, encoded in base64. Refer to the [Generate Wallet Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-wallet-token) section of our Authentication docs for more details on how to generate your Wallet Token.

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Path Parameters

The base58 encoded address of the Solana account.

#### Body

The arbitrary message to sign.

#### Response

Successfully signed message.

The signature of the message, as a base58 encoded string.

Example:

`"4YecmNqVT9QFqzuSvE9Zih3toZzNAijjXpj8xupgcC6E4VzwzFjuZBk5P99yz9JQaLRLm1K4L4FpMjxByFxQBe2h"`