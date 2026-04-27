# list loan assets

```
curl --request GET \
  --url https://api.exchange.coinbase.com/loans/assets \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
{
  "collateral_assets": {},
  "diversification_ratio": "<string>",
  "borrowable_assets": [
    "<string>"
  ]
}
```

Get a list of lendable assets, a map of assets we accept as collateral for loans, and the haircut weight.

```
curl --request GET \
  --url https://api.exchange.coinbase.com/loans/assets \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
{
  "collateral_assets": {},
  "diversification_ratio": "<string>",
  "borrowable_assets": [
    "<string>"
  ]
}
```

#### Authorizations

#### Response