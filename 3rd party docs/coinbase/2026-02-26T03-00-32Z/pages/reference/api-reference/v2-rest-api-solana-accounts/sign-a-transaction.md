# sign a transaction

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

A JWT signed using your Wallet Secret, encoded in base64. Refer to the [Generate Wallet Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-wallet-token) section of our Authentication docs for more details on how to generate your Wallet Token.

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Path Parameters

The base58 encoded address of the Solana account.

#### Body

The base64 encoded transaction to sign.

Example:

`"AQABAgIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQABAQECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8CBgMBAQAAAAIBAwQAAAAABgIAAAAAAAYDBQEBAAAGBAgAAAAABgUAAAAA6AMAAAAAAAAGBgUBAQEBBgcEAQAAAAYICgMBAQIDBgkCBgAAAAYKAwABAQEGCwMGAQEBBgwDAAABAQAAAAA="`

#### Response

Successfully signed transaction.

The base64 encoded signed transaction.

Example:

`"AQACAdSOvpk0UJXs/rQRXYKSI9hcR0bkGp24qGv6t0/M1XjcQpHf6AHwLcPjEtKQI7p/U0Zo98lnJ5/PZMfVq/0BAgIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQABAQECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8CBgMBAQAAAAIBAwQAAAAABgIAAAAAAAYDBQEBAAAGBAgAAAAABgUAAAAA6AMAAAAAAAAGBgUBAQEBBgcEAQAAAAYICgMBAQIDBgkCBgAAAAYKAwABAQEGCwMGAQEBBgwDAAABAQAAAAA="`