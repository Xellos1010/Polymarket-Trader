# withdraw to coinbase account

Withdraw to Coinbase account

```
curl --request POST \
  --url https://api.exchange.coinbase.com/withdrawals/coinbase-account \
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

Withdraws funds from the specified `profile_id` to a [www.coinbase.com](http://www.coinbase.com/) wallet.

POST

/

withdrawals

/

coinbase-account

Withdraw to Coinbase account

```
curl --request POST \
  --url https://api.exchange.coinbase.com/withdrawals/coinbase-account \
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

**Withdraw funds to a Coinbase account**You can move funds between your Coinbase accounts and your Coinbase Exchange trading accounts within your daily limits. Moving funds between Coinbase and Coinbase Exchange is instant and free. See the [Coinbase Accounts](https://developer.chrome.com/api-reference/exchange-api/rest-api/accounts/get-all-account-profile) section for retrieving your Coinbase accounts.

## API Key Permissions

This endpoint requires the “transfer” permission.

#### Authorizations

#### Body

#### Response