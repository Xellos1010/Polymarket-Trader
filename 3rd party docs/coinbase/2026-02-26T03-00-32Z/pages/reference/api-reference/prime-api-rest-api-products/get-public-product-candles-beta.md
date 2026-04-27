# get public product candles beta

Get Public Product Candles (Beta)

Get rates for a single product by product ID, grouped in buckets. This feature is in beta please reach out to your Coinbase Prime account manager for more information.

#### Path Parameters

The portfolio id requesting market data.

#### Query Parameters

start\_time

string<date-time>

required

Timestamp for starting range of aggregations

end\_time

string<date-time>

required

Timestamp for ending range of aggregations

The timeframe each candle represents.

Available options

:

`ONE_MINUTE`,

`FIVE_MINUTES`,

`FIFTEEN_MINUTES`,

`ONE_HOUR`,

`SIX_HOURS`,

`ONE_DAY`,

`THIRTY_MINUTES`,

`TWO_HOURS`,

`FOUR_HOURS`

#### Response

candles

Represents a single candle data point · object\[\]