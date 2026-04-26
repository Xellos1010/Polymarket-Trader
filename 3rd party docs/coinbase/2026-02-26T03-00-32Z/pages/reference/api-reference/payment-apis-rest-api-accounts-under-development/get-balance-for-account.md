# get balance for account

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The unique identifier of the account. The ID of the Account, which is a UUID prefixed by the string `account_`.

Example:

`"account_af2937b0-9846-4fe7-bfe9-ccc22d935114"`

The symbol of the asset. The symbol of the asset (e.g., btc, eth, usd, usdc).

Required string length: `1 - 42`

#### Response

Successfully got balance.

A balance of an asset.

An asset, e.g. fiat or crypto.

Example:

```
{  
  "symbol": "btc",  
  "type": "crypto",  
  "name": "Bitcoin",  
  "decimals": 8  
}
```

Amount details denominated in different assets.

-   The keys represent the asset symbols (e.g., "btc", "usd"), - Each value contains available and total amounts. - There will always be an entry for the asset specified in the `asset` field.