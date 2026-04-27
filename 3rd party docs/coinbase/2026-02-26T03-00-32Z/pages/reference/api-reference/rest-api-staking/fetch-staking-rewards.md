# fetch staking rewards

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Query Parameters

A limit on the number of objects to be returned. Limit can range from 1 to 100, and the default is 50.

A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next\_page value returned in a previous response to request subsequent results.

Maximum string length: `5000`

#### Body

The ID of the blockchain network.

Example:

`"ethereum-mainnet"`

The symbol of the asset for which the staking rewards are being fetched.

The onchain addresses for which the staking rewards are being fetched

Example:

`"[0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a]"`

start\_time

string<date-time>

required

The start time of this reward period

Example:

`"2024-07-21T00:00:00Z"`

end\_time

string<date-time>

required

The end time of this reward period

Example:

`"2024-07-21T00:00:00Z"`

format

enum<string>

default:usd

required

The format in which the rewards are to be fetched i.e native or in equivalent USD

Available options

:

`usd`,

`native`

#### Response

The list of staking rewards

True if this list has another page of items after this one that can be fetched.

The page token to be used to fetch the next page.