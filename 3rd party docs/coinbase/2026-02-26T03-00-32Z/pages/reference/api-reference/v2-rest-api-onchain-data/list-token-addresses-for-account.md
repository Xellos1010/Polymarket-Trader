# list token addresses for account

List token addresses for account

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The blockchain network to query.

Available options

:

`base`,

`base-sepolia`

The account address to analyze for token interactions.

#### Response

Token addresses retrieved successfully.

Response containing token addresses that an account has received.

The account address that was queried.

Example:

`"0x742d35Cc6634C0532925a3b844Bc454e4438f44e"`

List of token contract addresses that the account has received.

Example:

```
[  
  "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",  
  "0x4200000000000000000000000000000000000006",  
  "0x0000000000000000000000000000000000000000"  
]
```

Total number of unique token addresses discovered.