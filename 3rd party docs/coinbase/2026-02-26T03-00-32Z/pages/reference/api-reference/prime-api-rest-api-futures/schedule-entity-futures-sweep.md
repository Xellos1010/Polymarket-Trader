# schedule entity futures sweep

Schedule Entity Futures Sweep

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/sweeps \
  --header 'Content-Type: application/json' \
  --data '
{
  "currency": "USD",
  "amount": "1000.00"
}
'
```

```
{
  "success": true,
  "request_id": "00000000-0000-0000-0000-000000000000"
}
```

Schedule a sweep for a given entity from FCM wallet to USD Spot wallet. Only one pending sweep is allowed at a time per entity.

POST

/

v1

/

entities

/

{entity\_id}

/

futures

/

sweeps

Schedule Entity Futures Sweep

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/sweeps \
  --header 'Content-Type: application/json' \
  --data '
{
  "currency": "USD",
  "amount": "1000.00"
}
'
```

```
{
  "success": true,
  "request_id": "00000000-0000-0000-0000-000000000000"
}
```

#### Path Parameters

#### Body

Amount. Default to sweep all if not provided

#### Response

Request ID

Example:

`"00000000-0000-0000-0000-000000000000"`