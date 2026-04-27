# send a solana transaction

Send a Solana transaction

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

A JWT signed using your Wallet Secret, encoded in base64. Refer to the [Generate Wallet Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-wallet-token) section of our Authentication docs for more details on how to generate your Wallet Token.

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Body

The Solana network to send the transaction to.

Available options

:

`solana`,

`solana-devnet`

The base64 encoded transaction to sign and send. This transaction can contain multiple instructions for native Solana batching.

Example:

`"AQABAgIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQABAQECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8CBgMBAQAAAAIBAwQAAAAABgIAAAAAAAYDBQEBAAAGBAgAAAAABgUAAAAA6AMAAAAAAAAGBgUBAQEBBgcEAQAAAAYICgMBAQIDBgkCBgAAAAYKAwABAQEGCwMGAQEBBgwDAAABAQAAAAA="`

#### Response

Successfully signed and sent transaction.

The base58 encoded transaction signature.

Example:

`"5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW"`