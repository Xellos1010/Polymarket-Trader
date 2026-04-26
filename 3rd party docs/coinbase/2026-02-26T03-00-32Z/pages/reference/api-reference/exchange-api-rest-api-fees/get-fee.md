# get fee

```
curl --request GET \
  --url https://api.exchange.coinbase.com/fees \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
{
  "maker_fee_rate": "0.0050",
  "taker_fee_rate": "0.0050",
  "usd_volume": "43806.92"
}
```

Get fees rates and 30 days trailing volume.

```
curl --request GET \
  --url https://api.exchange.coinbase.com/fees \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
{
  "maker_fee_rate": "0.0050",
  "taker_fee_rate": "0.0050",
  "usd_volume": "43806.92"
}
```

This request returns your current maker & taker fee rates, as well as your 30-day trailing volume. Quoted rates are subject to change.

## API Key Permissions

This endpoint requires the “view” permission.

#### Authorizations

#### Response

Fees defines taker and maker fees for a given user including the volume in USD.

The 30 days trailing volume in USD.