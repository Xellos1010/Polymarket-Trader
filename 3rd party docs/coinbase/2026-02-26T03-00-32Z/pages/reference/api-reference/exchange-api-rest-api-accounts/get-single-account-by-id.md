# get single account by id

Get a single account by id

```
curl --request GET \
  --url https://api.exchange.coinbase.com/accounts/{account_id} \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
{
  "id": "7fd0abc0-e5ad-4cbb-8d54-f2b3f43364da",
  "currency": "USD",
  "balance": "0.0000000000000000",
  "hold": "0.0000000000000000",
  "available": "0",
  "profile_id": "8058d771-2d88-4f0f-ab6e-299c153d4308",
  "trading_enabled": true
}
```

Information for a single account. Use this endpoint when you know the account\_id. API key must belong to the same profile as the account.

Get a single account by id

```
curl --request GET \
  --url https://api.exchange.coinbase.com/accounts/{account_id} \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
{
  "id": "7fd0abc0-e5ad-4cbb-8d54-f2b3f43364da",
  "currency": "USD",
  "balance": "0.0000000000000000",
  "hold": "0.0000000000000000",
  "available": "0",
  "profile_id": "8058d771-2d88-4f0f-ab6e-299c153d4308",
  "trading_enabled": true
}
```

## API Key Permissions

This endpoint requires either the “view” or “trade” permission.

#### Authorizations

#### Path Parameters

#### Response

Amount in pending deposits transfers.