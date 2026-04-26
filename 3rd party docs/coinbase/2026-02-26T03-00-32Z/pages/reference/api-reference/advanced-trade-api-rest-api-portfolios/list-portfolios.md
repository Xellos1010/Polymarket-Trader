# list portfolios

Get all portfolios of a user.

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Query Parameters

portfolio\_type

enum<string>

default:UNDEFINED

Only returns portfolios matching this portfolio type.

Available options

:

`UNDEFINED`,

`DEFAULT`,

`CONSUMER`,

`INTX`

#### Response