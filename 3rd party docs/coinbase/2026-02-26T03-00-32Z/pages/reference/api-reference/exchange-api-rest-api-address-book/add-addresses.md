# add addresses

```
curl --request POST \
  --url https://api.exchange.coinbase.com/address-book \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "addresses": [
    {
      "label": "my wallet",
      "to": {
        "address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk"
      },
      "currency": "BTC"
    },
    {
      "label": "my XRP wallet",
      "to": {
        "address": "rEb8TK3gBgk5auZkwc6sHnwrGVJH8DuaLh",
        "destination_tag": "123"
      },
      "currency": "XRP"
    }
  ]
}
'
```

```
{
  "body": [
    {
      "id": "1",
      "address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk",
      "display_address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk",
      "address_info": {
        "address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk",
        "display_address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk"
      },
      "currency": "BTC",
      "label": "my wallet",
      "address_booked": true,
      "address_book_added_at": "2019-02-06T22:27:50.221Z"
    }
  ]
}
```

Add new addresses to address book

```
curl --request POST \
  --url https://api.exchange.coinbase.com/address-book \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "addresses": [
    {
      "label": "my wallet",
      "to": {
        "address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk"
      },
      "currency": "BTC"
    },
    {
      "label": "my XRP wallet",
      "to": {
        "address": "rEb8TK3gBgk5auZkwc6sHnwrGVJH8DuaLh",
        "destination_tag": "123"
      },
      "currency": "XRP"
    }
  ]
}
'
```

```
{
  "body": [
    {
      "id": "1",
      "address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk",
      "display_address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk",
      "address_info": {
        "address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk",
        "display_address": "1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk"
      },
      "currency": "BTC",
      "label": "my wallet",
      "address_booked": true,
      "address_book_added_at": "2019-02-06T22:27:50.221Z"
    }
  ]
}
```

#### Authorizations

#### Body

List of addresses to add to the address book

#### Response