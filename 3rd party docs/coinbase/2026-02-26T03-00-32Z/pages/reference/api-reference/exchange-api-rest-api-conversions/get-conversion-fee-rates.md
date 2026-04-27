# get conversion fee rates

```
curl --request GET \
  --url https://api.exchange.coinbase.com/conversions/fees \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "from_currency": "USDC",
    "to_currency": "USD",
    "fee_rate": "0.001",
    "thirty_day_volume": "1000000.00000000",
    "available_credit": "1000000.00000000"
  }
]
```

Gets a list of current conversion fee rates and trailing 30 day volume by currency pair

```
curl --request GET \
  --url https://api.exchange.coinbase.com/conversions/fees \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "from_currency": "USDC",
    "to_currency": "USD",
    "fee_rate": "0.001",
    "thirty_day_volume": "1000000.00000000",
    "available_credit": "1000000.00000000"
  }
]
```

#### Authorizations

#### Response