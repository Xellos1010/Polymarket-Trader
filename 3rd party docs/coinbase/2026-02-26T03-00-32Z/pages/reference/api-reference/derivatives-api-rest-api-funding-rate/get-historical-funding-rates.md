# get historical funding rates

Get historical funding rates

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/funding-rate
```

```
[
  {
    "symbol": "BIPZ30",
    "funding_rate": "0.000100",
    "future_mark_price": "62850.25",
    "spot_mark_price": "62840.10",
    "fair_value_price": "62845.00",
    "event_time": "2025-08-13T12:00:00Z"
  }
]
```

Retrieves the historical funding rate data for a specific instrument.

Get historical funding rates

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/funding-rate
```

```
[
  {
    "symbol": "BIPZ30",
    "funding_rate": "0.000100",
    "future_mark_price": "62850.25",
    "spot_mark_price": "62840.10",
    "fair_value_price": "62845.00",
    "event_time": "2025-08-13T12:00:00Z"
  }
]
```

#### Query Parameters

The instrument symbol (e.g., "BIPZ30")

Trading session date in YYYY-MM-DD format (defaults to today's date if absent)

#### Response

Successfully retrieved funding rate data

Example:

`"2025-08-13T12:00:00Z"`