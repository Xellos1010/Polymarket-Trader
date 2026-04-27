# list interest charges

```
curl --request GET \
  --url https://api.exchange.coinbase.com/loans/interest/{loan_id} \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "date": "2023-11-07T05:31:56Z",
    "currency": "<string>",
    "principal_amount": "<string>",
    "interest_rate": "<string>",
    "interest_accrued": "<string>"
  }
]
```

List interest charges for a loan

GET

/

loans

/

interest

/

{loan\_id}

```
curl --request GET \
  --url https://api.exchange.coinbase.com/loans/interest/{loan_id} \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "date": "2023-11-07T05:31:56Z",
    "currency": "<string>",
    "principal_amount": "<string>",
    "interest_rate": "<string>",
    "interest_accrued": "<string>"
  }
]
```

#### Authorizations

#### Path Parameters

#### Response