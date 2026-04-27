# list evm accounts

Lists the EVM accounts belonging to the developer’s CDP Project. The response is paginated, and by default, returns 20 accounts per page.

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Query Parameters

The number of resources to return per page.

The token for the next page of resources, if any.

#### Response

Successfully listed EVM accounts.

The list of EVM accounts.

The token for the next page of items, if any.

Example:

`"eyJsYXN0X2lkIjogImFiYzEyMyIsICJ0aW1lc3RhbXAiOiAxNzA3ODIzNzAxfQ=="`