# get counterparty address book

Get counterparty address book

```
curl --request GET \
  --url https://api.exchange.coinbase.com/address-book/counterparty \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "id": "22cc4cc1-3f03-4dc6-bc72-7094e9cf48f8",
    "address": "CBX7NWINZS",
    "label": "counterparty",
    "added_at": "2019-02-06T22:27:50.221Z"
  }
]
```

Get all counterparty addresses stored in the address book

GET

/

address-book

/

counterparty

Get counterparty address book

```
curl --request GET \
  --url https://api.exchange.coinbase.com/address-book/counterparty \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
[
  {
    "id": "22cc4cc1-3f03-4dc6-bc72-7094e9cf48f8",
    "address": "CBX7NWINZS",
    "label": "counterparty",
    "added_at": "2019-02-06T22:27:50.221Z"
  }
]
```

#### Authorizations

#### Response

added\_at

string<date-time>

required