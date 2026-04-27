# deposit from coinbase account

Deposit from Coinbase account

```
curl --request POST \
  --url https://api.exchange.coinbase.com/deposits/coinbase-account \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "coinbase_account_id": "<string>",
  "amount": "<string>",
  "currency": "<string>"
}
'
```

```
{
  "id": "<string>",
  "amount": "<string>",
  "currency": "<string>"
}
```

Deposits funds from a [www.coinbase.com](http://www.coinbase.com/) wallet to the specified `profile_id`.

POST

/

deposits

/

coinbase-account

Deposit from Coinbase account

```
curl --request POST \
  --url https://api.exchange.coinbase.com/deposits/coinbase-account \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "coinbase_account_id": "<string>",
  "amount": "<string>",
  "currency": "<string>"
}
'
```

```
{
  "id": "<string>",
  "amount": "<string>",
  "currency": "<string>"
}
```

**Deposit funds from a Coinbase account**You can move funds between your Coinbase accounts and your Coinbase Exchange trading accounts within your daily limits. Moving funds between Coinbase and Coinbase Exchange is instant and free. See [Get all Coinbase wallets](https://developer.chrome.com/api-reference/exchange-api/rest-api/coinbase-accounts/get-all-coinbase-wallets) for retrieving your Coinbase accounts.

## API Key Permissions

This endpoint requires the “transfer” permission.

#### Authorizations

#### Body

#### Response