# get public product candles

Get Public Product Candles

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The trading pair (e.g. 'BTC-USD').

#### Query Parameters

The UNIX timestamp indicating the start of the time interval.

The UNIX timestamp indicating the end of the time interval.

granularity

enum<string>

default:UNKNOWN\_GRANULARITY

required

The timeframe each candle represents.

Available options

:

`UNKNOWN_GRANULARITY`,

`ONE_MINUTE`,

`FIVE_MINUTE`,

`FIFTEEN_MINUTE`,

`THIRTY_MINUTE`,

`ONE_HOUR`,

`TWO_HOUR`,

`FOUR_HOUR`,

`SIX_HOUR`,

`ONE_DAY`

The number of candle buckets to be returned. By default, returns 350 (max 350).

#### Response