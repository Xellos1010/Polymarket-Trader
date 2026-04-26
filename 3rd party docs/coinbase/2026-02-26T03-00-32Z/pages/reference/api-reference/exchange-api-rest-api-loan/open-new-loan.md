# open new loan

```
curl --request POST \
  --url https://api.exchange.coinbase.com/loans/open \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "loan_id": "<string>",
  "currency": "<string>",
  "native_amount": "<string>",
  "interest_rate": "<string>",
  "term_start_date": "2023-11-07T05:31:56Z",
  "term_end_date": "2023-11-07T05:31:56Z",
  "profile_id": "<string>"
}
'
```

```
{
  "loan": {
    "id": "<string>",
    "currency": "<string>",
    "principal_amount": "<string>",
    "outstanding_principal_amount": "<string>",
    "interest_rate": "<string>",
    "interest_currency": "<string>",
    "status": "loan_pending",
    "effective_at": "2023-11-07T05:31:56Z",
    "closed_at": "2023-11-07T05:31:56Z",
    "term_start_date": "2023-11-07T05:31:56Z",
    "term_end_date": "2023-11-07T05:31:56Z"
  }
}
```

This API triggers a loan open request. Funding is not necessarily instantaneous and there is no SLA. You are notified when funds have settled in your Exchange account. Loan open requests, once initiated, cannot be canceled.

```
curl --request POST \
  --url https://api.exchange.coinbase.com/loans/open \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "loan_id": "<string>",
  "currency": "<string>",
  "native_amount": "<string>",
  "interest_rate": "<string>",
  "term_start_date": "2023-11-07T05:31:56Z",
  "term_end_date": "2023-11-07T05:31:56Z",
  "profile_id": "<string>"
}
'
```

```
{
  "loan": {
    "id": "<string>",
    "currency": "<string>",
    "principal_amount": "<string>",
    "outstanding_principal_amount": "<string>",
    "interest_rate": "<string>",
    "interest_currency": "<string>",
    "status": "loan_pending",
    "effective_at": "2023-11-07T05:31:56Z",
    "closed_at": "2023-11-07T05:31:56Z",
    "term_start_date": "2023-11-07T05:31:56Z",
    "term_end_date": "2023-11-07T05:31:56Z"
  }
}
```

#### Authorizations

#### Body

#### Response