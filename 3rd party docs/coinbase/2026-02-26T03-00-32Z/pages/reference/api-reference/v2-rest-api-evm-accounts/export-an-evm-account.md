# export an evm account

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

A JWT signed using your Wallet Secret, encoded in base64. Refer to the [Generate Wallet Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-wallet-token) section of our Authentication docs for more details on how to generate your Wallet Token.

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Path Parameters

The 0x-prefixed address of the EVM account. The address does not need to be checksummed.

#### Body

The base64-encoded, public part of the RSA key in DER format used to encrypt the account private key.

Example:

`"U2FsdGVkX1+vupppZksvRf5X5YgHq4+da+Q4qf51+Q4="`

#### Response

Successfully exported EVM account.

The base64-encoded, encrypted private key of the EVM account which is a 32 byte raw private key. The private key is encrypted in transport using the exportEncryptionKey in the request.

Example:

`"U2FsdGVkX1+vupppZksvRf5X5YgHq4+da+Q4qf51+Q4="`