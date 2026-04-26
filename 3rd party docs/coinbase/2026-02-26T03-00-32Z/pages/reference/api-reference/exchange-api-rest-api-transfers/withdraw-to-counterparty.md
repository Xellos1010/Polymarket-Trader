# withdraw to counterparty

```
curl --request POST \
  --url https://api.exchange.coinbase.com/withdrawals/counterparty \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "amount": "<string>",
  "counterparty_id": "<string>",
  "currency": "<string>"
}
'
```

```
{
  "id": "<string>",
  "counterparty_id": "<string>",
  "amount": "<string>",
  "currency": "<string>",
  "status": "UNKNOWN"
}
```

Withdraw funds from a specified currency account to a counterparty account

POST

/

withdrawals

/

counterparty

```
curl --request POST \
  --url https://api.exchange.coinbase.com/withdrawals/counterparty \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "amount": "<string>",
  "counterparty_id": "<string>",
  "currency": "<string>"
}
'
```

```
{
  "id": "<string>",
  "counterparty_id": "<string>",
  "amount": "<string>",
  "currency": "<string>",
  "status": "UNKNOWN"
}
```

## API Key Permissions

This endpoint requires the “transfer” permission. Withdrawals are restricted to the profile associated with the API key.

#### Authorizations

#### Body

#### Response

status

enum<string>

default:UNKNOWN

Available options

:

`UNKNOWN`,

`PENDING`,

`DONE`