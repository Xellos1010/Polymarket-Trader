# get product book

Get a list of bids/asks for a single product. The amount of detail shown can be customized with the limit parameter.

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Query Parameters

The trading pair (e.g. 'BTC-USD').

The number of bid/asks to be returned.

aggregation\_price\_increment

The minimum price intervals at which buy and sell orders are grouped or combined in the order book.

#### Response