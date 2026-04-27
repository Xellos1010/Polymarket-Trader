# list trade finance obligations

List Trade Finance Obligations

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/tf_obligations
```

```
{
  "obligations": [
    {
      "portfolio_id": "e8bbed13-fa33-41de-86d5-4335d8f08166",
      "symbol": "BTC",
      "amount_due": "150000",
      "notional_amount": "250000",
      "due_date": "1000"
    }
  ]
}
```

List trade finance obligations for a given entity.

GET

/

v1

/

entities

/

{entity\_id}

/

tf\_obligations

List Trade Finance Obligations

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/tf_obligations
```

```
{
  "obligations": [
    {
      "portfolio_id": "e8bbed13-fa33-41de-86d5-4335d8f08166",
      "symbol": "BTC",
      "amount_due": "150000",
      "notional_amount": "250000",
      "due_date": "1000"
    }
  ]
}
```

### Supported Products

-   Trade Finance

#### Path Parameters

The entity ID to retrieve obligations for

#### Response

obligations

Trade finance obligation information · object\[\]

The list of obligations (loans) for the entity.