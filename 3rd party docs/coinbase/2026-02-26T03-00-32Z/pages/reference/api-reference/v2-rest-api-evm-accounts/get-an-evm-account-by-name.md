# get an evm account by name

Get an EVM account by name

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The name of the EVM account.

#### Response

Successfully got EVM account.

The 0x-prefixed, checksum EVM address.

Example:

`"0x742d35Cc6634C0532925a3b844Bc454e4438f44e"`

An optional name for the account. Account names can consist of alphanumeric characters and hyphens, and be between 2 and 36 characters long. Account names are guaranteed to be unique across all EVM accounts in the developer's CDP Project.

The list of policy IDs that apply to the account. This will include both the project-level policy and the account-level policy, if one exists.

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