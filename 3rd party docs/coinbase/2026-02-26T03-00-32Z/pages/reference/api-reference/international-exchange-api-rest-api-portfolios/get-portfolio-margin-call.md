# get portfolio margin call

Get portfolio margin call status

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Path Parameters

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`)

#### Response

The status of the margin call

Available options

:

`NOT_LIQUIDATING`,

`MARGIN_CALL_PENDING`,

`LIQUIDATING`

Example:

`"MARGIN_CALL_PENDING"`

the ISO 8601 duration from a margin call start to expiry

details of an active margin call if there is one