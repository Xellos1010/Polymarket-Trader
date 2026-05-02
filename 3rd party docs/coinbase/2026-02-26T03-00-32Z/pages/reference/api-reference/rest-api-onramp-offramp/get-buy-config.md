# get buy config

The Buy Config API returns the list of countries supported by Coinbase Pay Onramp, and the payment methods available in each country. Clients should call this API periodically and cache the response.

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Response

List of supported countries and payment methods for buying

List of supported countries and the payment methods available in each country