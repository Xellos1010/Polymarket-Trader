# list solana token balances

List Solana token balances

Lists the token balances of a Solana address on a given network. The balances include SPL tokens and the native SOL token. The response is paginated, and by default, returns 20 balances per page.

**Note:** This endpoint is still under development and does not yet provide strong availability or freshness guarantees. Freshness and availability of new token balances will improve over the coming weeks.

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The base58 encoded Solana address to get balances for.

The human-readable network name to get the balances for. The name of the supported Solana networks in human-readable format.

Available options

:

`solana`,

`solana-devnet`

#### Query Parameters

The number of balances to return per page.

The token for the next page of balances. Will be empty if there are no more balances to fetch.

#### Response

Successfully listed token balances.

The list of Solana token balances.

Example:

```
[  
  {  
    "amount": { "amount": "1250000000", "decimals": 9 },  
    "token": {  
      "symbol": "SOL",  
      "name": "Solana",  
      "mintAddress": "So11111111111111111111111111111111111111111"  
    }  
  },  
  {  
    "amount": { "amount": "123456000", "decimals": 6 },  
    "token": {  
      "symbol": "USDC",  
      "name": "USD Coin",  
      "mintAddress": "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"  
    }  
  }  
]
```

The token for the next page of items, if any.

Example:

`"eyJsYXN0X2lkIjogImFiYzEyMyIsICJ0aW1lc3RhbXAiOiAxNzA3ODIzNzAxfQ=="`