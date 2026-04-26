# get fcm margin call details

Get FCM Margin Call Details

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/margin_call_details
```

```
{
  "margin_calls": [
    {
      "type": "FCM_MARGIN_CALL_TYPE_UNSPECIFIED",
      "state": "FCM_MARGIN_CALL_STATE_UNSPECIFIED",
      "initial_amount": "1000.00",
      "remaining_amount": "500.00",
      "business_date": "2021-01-01T00:00:00.000Z",
      "cure_deadline": "2021-01-01T00:00:00.000Z"
    }
  ]
}
```

Retrieve the margin call details for a given entity.

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

margin\_call\_details

Get FCM Margin Call Details

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/margin_call_details
```

```
{
  "margin_calls": [
    {
      "type": "FCM_MARGIN_CALL_TYPE_UNSPECIFIED",
      "state": "FCM_MARGIN_CALL_STATE_UNSPECIFIED",
      "initial_amount": "1000.00",
      "remaining_amount": "500.00",
      "business_date": "2021-01-01T00:00:00.000Z",
      "cure_deadline": "2021-01-01T00:00:00.000Z"
    }
  ]
}
```

#### Path Parameters

#### Response