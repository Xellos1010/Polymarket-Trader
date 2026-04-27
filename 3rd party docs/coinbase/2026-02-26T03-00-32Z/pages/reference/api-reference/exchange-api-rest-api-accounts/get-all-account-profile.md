# get all account profile

Get all accounts for a profile

```
curl --request GET \
  --url https://api.exchange.coinbase.com/accounts \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "id": "7fd0abc0-e5ad-4cbb-8d54-f2b3f43364da",
    "currency": "USD",
    "balance": "0.0000000000000000",
    "hold": "0.0000000000000000",
    "available": "0",
    "profile_id": "8058d771-2d88-4f0f-ab6e-299c153d4308",
    "trading_enabled": true
  }
]
```

Get a list of trading accounts from the profile of the API key.

Get all accounts for a profile

```
curl --request GET \
  --url https://api.exchange.coinbase.com/accounts \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "id": "7fd0abc0-e5ad-4cbb-8d54-f2b3f43364da",
    "currency": "USD",
    "balance": "0.0000000000000000",
    "hold": "0.0000000000000000",
    "available": "0",
    "profile_id": "8058d771-2d88-4f0f-ab6e-299c153d4308",
    "trading_enabled": true
  }
]
```

**Info**Your trading accounts are separate from your Coinbase accounts. See [Deposit from Coinbase account](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/deposit-from-coinbase-account) for documentation on how to deposit funds to begin trading.

## API Key Permissions

This endpoint requires either the “view” or “trade” permission.

## Rate Limits

This endpoint has a custom rate limit by profile ID: 25 requests per second, up to 50 requests per second in bursts

#### Authorizations

#### Response

Amount in pending deposits transfers.