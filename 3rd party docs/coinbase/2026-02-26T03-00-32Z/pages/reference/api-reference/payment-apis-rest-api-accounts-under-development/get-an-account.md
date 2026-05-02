# get an account

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The ID of the account to retrieve. The ID of the Account, which is a UUID prefixed by the string `account_`.

Example:

`"account_af2937b0-9846-4fe7-bfe9-ccc22d935114"`

#### Response

Successfully got account.

The ID of the Account, which is a UUID prefixed by the string `account_`.

Example:

`"account_af2937b0-9846-4fe7-bfe9-ccc22d935114"`

The type of the Account.

Available options

:

`prime`,

`business`,

`cdp`

The Owner ID, which can be either your Entity or a Customer of yours. Owner IDs are UUIDs prefixed with the Owner Type as follows:

-   Entity: `entity_` - If the Owner is your Entity, e.g. `entity_af2937b0-9846-4fe7-bfe9-ccc22d935114`
-   Customer: `customer_` - If the Owner is a Customer of yours, e.g. `customer_af2937b0-9846-4fe7-bfe9-ccc22d935114`.

Example:

`"customer_af2937b0-9846-4fe7-bfe9-ccc22d935114"`

createdAt

string<date-time>

required

The timestamp when the account was created.

Example:

`"2023-10-08T14:30:00Z"`

updatedAt

string<date-time>

required

The timestamp when the account was last updated.

Example:

`"2023-10-08T14:30:00Z"`

An optional name for the account. Must be 1-64 characters and can only contain alphanumeric characters, hyphens, and spaces.

Maximum string length: `64`

Example:

`"My Business Account"`