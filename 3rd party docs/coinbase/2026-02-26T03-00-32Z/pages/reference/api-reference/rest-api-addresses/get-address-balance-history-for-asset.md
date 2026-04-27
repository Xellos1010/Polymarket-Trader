# get address balance history for asset

Get address balance history for asset

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

Blockchain address hash (Note that EVM chain address hash should be lowered cased).

The symbol of the asset to fetch the historical balance for.

#### Query Parameters

A limit on the number of objects to be returned. The default value is 10. The maximum value is 100, and values supplied over this will be coerced to the maximum.

A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next\_page value returned in a previous response to request subsequent results.

Maximum string length: `5000`

#### Response

True if this list has another page of items after this one that can be fetched.

A token which can be provided as `page` token to retrieve the next page. If this field is omitted, there are no additional pages.