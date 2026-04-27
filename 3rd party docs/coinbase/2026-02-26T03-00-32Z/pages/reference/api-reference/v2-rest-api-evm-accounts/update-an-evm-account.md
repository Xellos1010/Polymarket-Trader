# update an evm account

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Path Parameters

The 0x-prefixed address of the EVM account. The address does not need to be checksummed.

#### Body

An optional name for the account. Account names can consist of alphanumeric characters and hyphens, and be between 2 and 36 characters long. Account names must be unique across all EVM accounts in the developer's CDP Project.

The ID of the account-level policy to apply to the account, or an empty string to unset attached policy.

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

#### Response

Successfully updated EVM account.

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