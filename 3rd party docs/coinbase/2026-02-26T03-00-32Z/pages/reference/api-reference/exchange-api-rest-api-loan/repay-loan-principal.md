# repay loan principal

```
curl --request POST \
  --url https://api.exchange.coinbase.com/loans/repay-principal \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "loan_id": "<string>",
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
    "loan_id": "<string>",
    "native_amount": "<string>",
    "initial_native_amount": "<string>",
    "status": "REPAYMENT_UNSET",
    "type": "REPAYMENT_TYPE_UNSET"
  }
}
```

Submit a principal repayment for a loan.

```
curl --request POST \
  --url https://api.exchange.coinbase.com/loans/repay-principal \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "loan_id": "<string>",
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
    "loan_id": "<string>",
    "native_amount": "<string>",
    "initial_native_amount": "<string>",
    "status": "REPAYMENT_UNSET",
    "type": "REPAYMENT_TYPE_UNSET"
  }
}
```

#### Authorizations

#### Body

#### Response