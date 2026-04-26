# get entity locate availabilities

Get Entity Locate Availabilities

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/locates_availability
```

```
{
  "locates": [
    {
      "symbol": "BTC",
      "quantity": "100",
      "rate": "0.05"
    }
  ]
}
```

Get currencies available to be located with their corresponding amount and rate.

GET

/

v1

/

entities

/

{entity\_id}

/

locates\_availability

Get Entity Locate Availabilities

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/locates_availability
```

```
{
  "locates": [
    {
      "symbol": "BTC",
      "quantity": "100",
      "rate": "0.05"
    }
  ]
}
```

### Supported Products

-   Portfolio Margin

#### Path Parameters

The unique ID of the entity

#### Query Parameters

Deprecated: Use locate\_date instead

The date of the locate availability in YYYY-MM-DD format

#### Response