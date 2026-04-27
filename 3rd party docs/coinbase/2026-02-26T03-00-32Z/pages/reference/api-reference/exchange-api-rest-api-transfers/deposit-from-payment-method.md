# deposit from payment method

Deposit from payment method

```
curl --request POST \
  --url https://api.exchange.coinbase.com/deposits/payment-method \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "amount": "<string>",
  "payment_method_id": "<string>",
  "currency": "<string>"
}
'
```

```
{
  "id": "<string>",
  "amount": "<string>",
  "currency": "<string>",
  "payout_at": "<string>",
  "fee": "<string>"
}
```

Deposits funds from a linked external payment method to the specified `profile_id`.

Deposit from payment method

```
curl --request POST \
  --url https://api.exchange.coinbase.com/deposits/payment-method \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "amount": "<string>",
  "payment_method_id": "<string>",
  "currency": "<string>"
}
'
```

```
{
  "id": "<string>",
  "amount": "<string>",
  "currency": "<string>",
  "payout_at": "<string>",
  "fee": "<string>"
}
```

**Deposit funds from a payment method**See [Get all payment methods](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/get-all-payment-methods). The SEPA payment method is not allowed for depositing funds because it is a push payment method.

## API Key Permissions

This endpoint requires the “transfer” permission. API key must belong to default profile.

#### Authorizations

#### Body

#### Response