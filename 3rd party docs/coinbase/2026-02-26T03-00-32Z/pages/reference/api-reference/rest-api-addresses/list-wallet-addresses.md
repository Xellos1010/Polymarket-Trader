# list wallet addresses

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The ID of the wallet whose addresses to fetch.

#### Query Parameters

A limit on the number of objects to be returned. Limit can range from 1 to 100, and the default is 10.

A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next\_page value returned in a previous response to request subsequent results.

Maximum string length: `5000`

#### Response

True if this list has another page of items after this one that can be fetched.

The page token to be used to fetch the next page.

The total number of addresses for the wallet.