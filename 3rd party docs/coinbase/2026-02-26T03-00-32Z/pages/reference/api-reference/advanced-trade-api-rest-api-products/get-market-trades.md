# get market trades

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The trading pair (e.g. 'BTC-USD').

#### Query Parameters

The number of trades to be returned.

The UNIX timestamp indicating the start of the time interval.

The UNIX timestamp indicating the end of the time interval.

#### Response

The best bid for the `product_id`, in quote currency.

The best ask for the `product_id`, in quote currency.