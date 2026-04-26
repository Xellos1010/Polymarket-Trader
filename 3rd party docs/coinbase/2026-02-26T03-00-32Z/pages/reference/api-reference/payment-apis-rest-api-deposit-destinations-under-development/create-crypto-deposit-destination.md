# create crypto deposit destination

Create crypto deposit destination

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Body

Request to create a new cryptocurrency deposit destination.

The ID of the Account, which is a UUID prefixed by the string `account_`.

Example:

`"account_af2937b0-9846-4fe7-bfe9-ccc22d935114"`

The type of deposit destination. Only crypto destinations can be created via the API.

Available options

:

`crypto`

The blockchain network for the payment. All networks support USDC and USDT.

Available options

:

`base`,

`ethereum`,

`solana`,

`aptos`,

`arbitrum`,

`optimism`,

`polygon`,

`sui`

Optional metadata as key-value pairs. Use this to store additional structured information on a resource, such as customer IDs, order references, or any application-specific data. Up to 50 key/value pairs may be provided. Keys and values are both strings. Keys must be ≤ 40 characters; values must be ≤ 500 characters.

Example:

```
{  
  "customer_id": "cust_12345",  
  "order_reference": "order-67890"  
}
```

#### Response

Successfully created deposit destination.

A deposit destination for receiving funds to an account. Can be either a cryptocurrency address or fiat bank account instructions.

The ID of the Deposit Destination, which is a UUID prefixed by the string `depositDestination_`.

Example:

`"depositDestination_af2937b0-9846-4fe7-bfe9-ccc22d935114"`

The ID of the Account, which is a UUID prefixed by the string `account_`.

Example:

`"account_af2937b0-9846-4fe7-bfe9-ccc22d935114"`

The type of deposit destination.

Available options

:

`crypto`

The cryptocurrency address where funds can be deposited. Format depends on the network (e.g., 0x-prefixed for EVM networks, base58 for Solana).

Required string length: `1 - 128`

Example:

`"0x742d35Cc6634C0532925a3b844Bc454e4438f44e"`

The blockchain network for the payment. All networks support USDC and USDT.

Available options

:

`base`,

`ethereum`,

`solana`,

`aptos`,

`arbitrum`,

`optimism`,

`polygon`,

`sui`

createdAt

string<date-time>

required

The timestamp when the deposit destination was created.

Example:

`"2023-10-08T14:30:00Z"`

updatedAt

string<date-time>

required

The timestamp when the deposit destination was last updated.

Example:

`"2023-10-08T14:30:00Z"`

Optional metadata as key-value pairs. Use this to store additional structured information on a resource, such as customer IDs, order references, or any application-specific data. Up to 50 key/value pairs may be provided. Keys and values are both strings. Keys must be ≤ 40 characters; values must be ≤ 500 characters.

Example:

```
{  
  "customer_id": "cust_12345",  
  "order_reference": "order-67890"  
}
```