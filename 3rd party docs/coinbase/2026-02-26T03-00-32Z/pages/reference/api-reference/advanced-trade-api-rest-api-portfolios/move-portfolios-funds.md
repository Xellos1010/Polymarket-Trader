# move portfolios funds

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Body

The amount to be moved to the specified portfolio.

The UUID of the portfolio to send funds from.

Example:

`"8bfc20d7-f7c6-4422-bf07-8243ca4169fe"`

The UUID of the portfolio to send funds to.

Example:

`"8bfc20d7-f7c6-4422-bf07-8243ca4169fe"`

#### Response

The UUID of the portfolio to send funds from.

Example:

`"8bfc20d7-f7c6-4422-bf07-8243ca4169fe"`

The UUID of the portfolio to send funds to.

Example:

`"8bfc20d7-f7c6-4422-bf07-8243ca4169fe"`