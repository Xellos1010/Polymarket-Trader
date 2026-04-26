# create a smart account

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Body

Today, only a single owner can be set for a Smart Account, but this is an array to allow setting multiple owners in the future.

Example:

```
[  
  "0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a"  
]
```

An optional name for the account. Account names can consist of alphanumeric characters and hyphens, and be between 2 and 36 characters long. Account names must be unique across all EVM accounts in the developer's CDP Project.

Example:

`"my-smart-wallet"`

#### Response

Successfully created Smart Account.

The 0x-prefixed, checksum address of the Smart Account.

Example:

`"0x742d35Cc6634C0532925a3b844Bc454e4438f44e"`

Today, only a single owner can be set for a Smart Account, but this is an array to allow having multiple owners in the future. The address is a 0x-prefixed, checksum address.

Example:

```
[  
  "0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a"  
]
```

An optional name for the account. Account names can consist of alphanumeric characters and hyphens, and be between 2 and 36 characters long. Account names are guaranteed to be unique across all Smart Accounts in the developer's CDP Project.

Example:

`"my-smart-account"`

The list of policy IDs that apply to the smart account. This will include both the project-level policy and the account-level policy, if one exists.

Example:

```
["123e4567-e89b-12d3-a456-426614174000"]
```

The UTC ISO 8601 timestamp at which the account was created.

Example:

`"2025-03-25T12:00:00Z"`

The UTC ISO 8601 timestamp at which the account was last updated.

Example:

`"2025-03-26T12:00:00Z"`