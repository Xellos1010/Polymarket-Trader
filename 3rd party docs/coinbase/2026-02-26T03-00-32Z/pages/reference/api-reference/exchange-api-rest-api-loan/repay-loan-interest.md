# repay loan interest

```
curl --request POST \
  --url https://api.exchange.coinbase.com/loans/repay-interest \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "idem": "<string>",
  "from_profile_id": "<string>",
  "currency": "<string>",
  "native_amount": "<string>"
}
'
```

```
{
  "repayment": {
    "id": "<string>",
    "native_amount": "<string>",
    "status": "REPAYMENT_UNSET",
    "type": "REPAYMENT_TYPE_UNSET"
  }
}
```

Submit an interest repayment for a loan.

```
curl --request POST \
  --url https://api.exchange.coinbase.com/loans/repay-interest \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "idem": "<string>",
  "from_profile_id": "<string>",
  "currency": "<string>",
  "native_amount": "<string>"
}
'
```

```
{
  "repayment": {
    "id": "<string>",
    "native_amount": "<string>",
    "status": "REPAYMENT_UNSET",
    "type": "REPAYMENT_TYPE_UNSET"
  }
}
```

#### Authorizations

#### Body

#### Response