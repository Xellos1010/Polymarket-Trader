# list advanced transfers

```
{
  "advanced_transfers": [
    {
      "id": "<string>",
      "type": "ADVANCED_TRANSFER_TYPE_BLIND_MATCH",
      "state": "ADVANCED_TRANSFER_STATE_CREATED",
      "fund_movements": [
        {
          "id": "<string>",
          "source": {
            "type": "PAYMENT_METHOD",
            "value": "0bf7bf1e-bafa-4d7e-9312-fa0bf3b63f27",
            "address": 6.6325114945411165e+47,
            "account_identifier": "387879289"
          },
          "target": {
            "type": "PAYMENT_METHOD",
            "value": "0bf7bf1e-bafa-4d7e-9312-fa0bf3b63f27",
            "address": 6.6325114945411165e+47,
            "account_identifier": "387879289"
          },
          "currency": "<string>",
          "amount": "<string>"
        }
      ],
      "blind_match_metadata": {
        "reference_id": "<string>",
        "settlement_date": "<string>",
        "trade_date": "<string>",
        "settlement_time": "<string>"
      }
    }
  ],
  "pagination": {
    "next_cursor": "<string>",
    "sort_direction": "DESC",
    "has_next": true
  }
}
```