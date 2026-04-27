# allocate portfolio

International Derivatives

Allocate portfolio funds to a sub-portfolio on Intx Portfolio

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Body

The trading pair (e.g. 'BTC-PERP-INTX').

The amount to be allocated for the specified isolated position.

The currency to be allocated for the specific isolated position (e.g. USD, BTC, etc).

#### Response

The response is of type `object`.