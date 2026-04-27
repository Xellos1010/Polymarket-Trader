# cancel order

```
curl --request POST \
  --url https://api.coinbase.com/api/v3/brokerage/orders/batch_cancel \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "order_ids": [
    "0000-00000",
    "1111-11111"
  ]
}
'
```

```
{
  "results": [
    {
      "success": true,
      "failure_reason": "UNKNOWN_CANCEL_FAILURE_REASON",
      "order_id": "0000-00000"
    }
  ]
}
```

Initiate cancel requests for one or more orders.

POST

/

api

/

v3

/

brokerage

/

orders

/

batch\_cancel

```
curl --request POST \
  --url https://api.coinbase.com/api/v3/brokerage/orders/batch_cancel \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "order_ids": [
    "0000-00000",
    "1111-11111"
  ]
}
'
```

```
{
  "results": [
    {
      "success": true,
      "failure_reason": "UNKNOWN_CANCEL_FAILURE_REASON",
      "order_id": "0000-00000"
    }
  ]
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Body

The order IDs that cancel requests should be initiated for.

Example:

```
["0000-00000", "1111-11111"]
```

#### Response

The result of initiated cancel requests