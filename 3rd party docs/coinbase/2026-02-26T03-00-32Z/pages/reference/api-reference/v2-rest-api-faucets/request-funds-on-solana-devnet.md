# request funds on solana devnet

Request funds on Solana devnet

```
{  
  "transactionSignature": "4dje1d24iG2FfxwxTJJt8VSTtYXNc6AAuJwngtL97TJSqqPD3pgRZ7uh4szoU6WDrKyFTBgaswkDrCr7BqWjQqqK"  
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Body

The address to request funds to, which is a base58-encoded string.

Example:

`"HpabPRRCFbBKSuJr5PdkVvQc85FyxyTWkFM2obBRSvHT"`

The token to request funds for.

Available options

:

`sol`,

`usdc`

#### Response

Successfully requested funds.

The signature identifying the transaction that requested the funds.

Example:

`"4dje1d24iG2FfxwxTJJt8VSTtYXNc6AAuJwngtL97TJSqqPD3pgRZ7uh4szoU6WDrKyFTBgaswkDrCr7BqWjQqqK"`