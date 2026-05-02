# get api key permissions

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Response

Indicates whether the API key has view permissions.

Indicates whether the API key has trade permissions.

Indicates whether the API key has deposit/withdrawal permissions.

The portfolio ID associated with the API key.

portfolio\_type

enum<string>

default:UNDEFINED

The type of portfolio

Available options

:

`UNDEFINED`,

`DEFAULT`,

`CONSUMER`,

`INTX`