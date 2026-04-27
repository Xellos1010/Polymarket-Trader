# list portfolio allocations

```
{
  "allocations": [
    {
      "root_id": "<string>",
      "reversal_id": "<string>",
      "allocation_completed_at": "2023-11-07T05:31:56Z",
      "user_id": "<string>",
      "product_id": "<string>",
      "side": "BUY",
      "avg_price": "<string>",
      "base_quantity": "<string>",
      "quote_value": "<string>",
      "fees_allocated": "<string>",
      "status": "ALLOCATION_STATUS_UNSPECIFIED",
      "source": "<string>",
      "order_ids": [
        "<string>"
      ],
      "destinations": [
        {
          "leg_id": "<string>",
          "portfolio_id": "<string>",
          "allocation_base": "<string>",
          "allocation_quote": "<string>",
          "fees_allocated_leg": "<string>"
        }
      ],
      "netting_id": "<string>"
    }
  ],
  "pagination": {
    "next_cursor": "<string>",
    "sort_direction": "DESC",
    "has_next": true
  }
}
```