# get offramp transactions by id

```
{
  "next_page_key": "<string>",
  "total_count": 123,
  "transactions": [
    {
      "asset": "<string>",
      "coinbase_fee": {
        "currency": "<string>",
        "value": "<string>"
      },
      "created_at": "2023-11-07T05:31:56Z",
      "exchange_rate": {
        "currency": "<string>",
        "value": "<string>"
      },
      "from_address": "<string>",
      "minimum_total": {
        "currency": "<string>",
        "value": "<string>"
      },
      "network": "<string>",
      "redirect_url": "<string>",
      "sell_amount": {
        "currency": "<string>",
        "value": "<string>"
      },
      "status": "TRANSACTION_STATUS_CREATED",
      "subtotal": {
        "currency": "<string>",
        "value": "<string>"
      },
      "to_address": "<string>",
      "total": {
        "currency": "<string>",
        "value": "<string>"
      },
      "transaction_id": "<string>",
      "tx_hash": "<string>",
      "unit_price": {
        "currency": "<string>",
        "value": "<string>"
      },
      "updated_at": "2023-11-07T05:31:56Z",
      "payment_method": "UNSPECIFIED"
    }
  ]
}
```