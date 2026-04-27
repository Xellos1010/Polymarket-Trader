# get unstaking status

```
{
  "portfolio_id": "<string>",
  "wallet_id": "<string>",
  "wallet_address": "<string>",
  "current_timestamp": "2025-10-17T15:30:00.000Z",
  "validators": [
    {
      "validator_address": "<string>",
      "statuses": [
        {
          "amount": "16",
          "estimate_type": "UNSPECIFIED",
          "estimate_description": "Live estimate based on current network conditions",
          "unstake_type": "UNSTAKE_TYPE_UNSPECIFIED",
          "finishing_at": "2025-10-27T00:00:00.000Z",
          "remaining_hours": 672,
          "requested_at": "2025-09-29T12:00:00.000Z"
        }
      ]
    }
  ]
}
```