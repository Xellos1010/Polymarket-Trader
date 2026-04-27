# list accounts

List all accounts. The API will return all accounts that the API Key has Permissions to access. You can filter the results by using query parameters, which will be treated as a single conjunction (i.e. AND). Results are sorted by creation date in descending order (newest first).

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Query Parameters

The number of resources to return per page.

The token for the next page of resources, if any.

#### Response

Successfully listed accounts.

The token for the next page of items, if any.

Example:

`"eyJsYXN0X2lkIjogImFiYzEyMyIsICJ0aW1lc3RhbXAiOiAxNzA3ODIzNzAxfQ=="`