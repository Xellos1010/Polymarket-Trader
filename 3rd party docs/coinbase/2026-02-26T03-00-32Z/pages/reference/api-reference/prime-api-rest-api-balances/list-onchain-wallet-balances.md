# list onchain wallet balances

```
{
  "balances": [
    {
      "asset": {
        "network": "<string>",
        "contract_address": "<string>",
        "symbol": "<string>",
        "token_id": "<string>",
        "name": "<string>"
      },
      "amount": "109.42",
      "visibility_status": "VISIBLE"
    }
  ],
  "pagination": {
    "next_cursor": "<string>",
    "sort_direction": "DESC",
    "has_next": true
  },
  "defi_balances": [
    {
      "network": "<string>",
      "protocol": "<string>",
      "net_usd_value": "<string>"
    }
  ]
}
```