# list entity payment methods

List Entity Payment Methods

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/payment-methods
```

```
{
  "payment_methods": [
    {
      "id": "<string>",
      "symbol": "<string>",
      "payment_method_type": "METHOD_WIRE",
      "bank_name": "<string>",
      "account_number": "<string>",
      "bank_name_2": "<string>"
    }
  ]
}
```

Retrieve all payment methods for a given entity.

GET

/

v1

/

entities

/

{entity\_id}

/

payment-methods

List Entity Payment Methods

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/payment-methods
```

```
{
  "payment_methods": [
    {
      "id": "<string>",
      "symbol": "<string>",
      "payment_method_type": "METHOD_WIRE",
      "bank_name": "<string>",
      "account_number": "<string>",
      "bank_name_2": "<string>"
    }
  ]
}
```

#### Path Parameters

#### Response