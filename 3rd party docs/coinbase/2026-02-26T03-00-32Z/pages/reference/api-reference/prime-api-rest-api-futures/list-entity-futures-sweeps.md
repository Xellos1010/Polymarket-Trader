# list entity futures sweeps

List Entity Futures Sweeps

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/sweeps
```

```
{
  "sweeps": [
    {
      "id": "00000000-0000-0000-0000-000000000000",
      "requested_amount": {
        "currency": "USD",
        "amount": "1000.00"
      },
      "should_sweep_all": true,
      "status": "FCM_FUTURES_SWEEP_STATUS_UNSPECIFIED",
      "scheduled_time": "2021-01-01T00:00:00.000Z"
    }
  ],
  "auto_sweep": true
}
```

Retrieve fcm sweeps in open status, including pending and processing sweeps.

GET

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

List Entity Futures Sweeps

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/sweeps
```

```
{
  "sweeps": [
    {
      "id": "00000000-0000-0000-0000-000000000000",
      "requested_amount": {
        "currency": "USD",
        "amount": "1000.00"
      },
      "should_sweep_all": true,
      "status": "FCM_FUTURES_SWEEP_STATUS_UNSPECIFIED",
      "scheduled_time": "2021-01-01T00:00:00.000Z"
    }
  ],
  "auto_sweep": true
}
```

### Pending vs. Processing Sweeps

-   A pending sweep is a sweep that has not started processing and can be cancelled
-   A processing sweep is a sweep that is currently being processed and cannot be cancelled

Once a sweep is complete, it no longer appears in the list of sweeps.

#### Path Parameters

#### Response