# list interest rate history

List interest rate history

```
curl --request GET \
  --url https://api.exchange.coinbase.com/loans/interest/history/{loan_id} \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "interest_rate": "<string>",
    "effective_at": "2023-11-07T05:31:56Z"
  }
]
```

List interest rate history for a loan

GET

/

loans

/

interest

/

history

/

{loan\_id}

List interest rate history

```
curl --request GET \
  --url https://api.exchange.coinbase.com/loans/interest/history/{loan_id} \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "interest_rate": "<string>",
    "effective_at": "2023-11-07T05:31:56Z"
  }
]
```

#### Authorizations

#### Path Parameters

#### Response