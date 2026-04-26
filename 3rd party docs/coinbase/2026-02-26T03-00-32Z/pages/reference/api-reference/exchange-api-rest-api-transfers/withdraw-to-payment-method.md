# withdraw to payment method

Withdraw to payment method

```
curl --request POST \
  --url https://api.exchange.coinbase.com/withdrawals/payment-method \
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

Withdraws funds from the specified `profile_id` to a linked external payment method

POST

/

withdrawals

/

payment-method

Withdraw to payment method

```
curl --request POST \
  --url https://api.exchange.coinbase.com/withdrawals/payment-method \
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

**Withdraw funds to a payment method**See the [Payment Methods](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/get-all-payment-methods) section for retrieving your payment methods.

## API Key Permissions

This endpoint requires the “transfer” permission. API key is restricted to the default profile.

#### Authorizations

#### Body

#### Response