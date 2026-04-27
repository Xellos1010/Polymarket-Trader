# list wallet balances

List the balances of all of the addresses in the wallet aggregated by asset.

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The ID of the wallet to fetch the balances for.

#### Response

True if this list has another page of items after this one that can be fetched.

The page token to be used to fetch the next page.

The total number of balances for the wallet.