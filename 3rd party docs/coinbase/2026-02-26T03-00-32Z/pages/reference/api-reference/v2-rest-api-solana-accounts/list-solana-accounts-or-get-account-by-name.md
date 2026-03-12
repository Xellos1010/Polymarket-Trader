# list solana accounts or get account by name

List Solana accounts or get account by name

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Query Parameters

The number of resources to return per page.

The token for the next page of resources, if any.

#### Response

Successfully listed Solana accounts.

The list of Solana accounts.

The token for the next page of items, if any.

Example:

`"eyJsYXN0X2lkIjogImFiYzEyMyIsICJ0aW1lc3RhbXAiOiAxNzA3ODIzNzAxfQ=="`