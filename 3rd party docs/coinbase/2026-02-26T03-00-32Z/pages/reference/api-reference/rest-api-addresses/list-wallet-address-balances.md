# list wallet address balances

List wallet address balances

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The ID of the wallet to fetch the balances for.

The onchain address of the address that is being fetched.

#### Query Parameters

A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next\_page value returned in a previous response to request subsequent results.

Maximum string length: `5000`

#### Response

The paginated list of balances

True if this list has another page of items after this one that can be fetched.

A token which can be provided as `page` token to retrieve the next page. If this field is omitted, there are no additional pages.

The total number of balances for the wallet.