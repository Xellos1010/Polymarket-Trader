# get address book

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Query Parameters

Filter by recipient type

Available options

:

`CRYPTO_ADDRESS`,

`COUNTERPARTY_ID`

#### Response

The type of withdrawal recipient

Available options

:

`CRYPTO_ADDRESS`,

`COUNTERPARTY_ID`

The crypto address or counterparty ID

Example:

`"0x29d2D586e222D0610b04e71974699589379F13b5"`

User-defined label to categorize the address

User-defined nickname for the address

The status of the address allowlist entry. PENDING entries become ACTIVE after 24 hours

Available options

:

`ACTIVE`,

`DISABLED`,

`PENDING`

The name of the asset (for crypto addresses only)

The blockchain network identifier (for crypto addresses only)

Example:

`"networks/ethereum-mainnet/assets/313ef8a9-ae5a-5f2f-8a56-572c0e2a4d5a"`

The time the address was added to the allowlist

Example:

`"2023-09-16T23:59:53.000Z"`