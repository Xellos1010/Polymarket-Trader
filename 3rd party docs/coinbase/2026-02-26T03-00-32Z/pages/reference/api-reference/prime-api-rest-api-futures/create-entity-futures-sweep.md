# create entity futures sweep

Cancel Entity Futures Sweep

```
curl --request DELETE \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/sweeps
```

```
{
  "success": true,
  "request_id": "00000000-0000-0000-0000-000000000000"
}
```

Cancel the pending sweep for a given entity. A user will only be able to have one pending sweep at a time. If the sweep is not found, a 404 will be returned.

DELETE

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

Cancel Entity Futures Sweep

```
curl --request DELETE \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/sweeps
```

```
{
  "success": true,
  "request_id": "00000000-0000-0000-0000-000000000000"
}
```

#### Path Parameters

#### Response

Request ID

Example:

`"00000000-0000-0000-0000-000000000000"`