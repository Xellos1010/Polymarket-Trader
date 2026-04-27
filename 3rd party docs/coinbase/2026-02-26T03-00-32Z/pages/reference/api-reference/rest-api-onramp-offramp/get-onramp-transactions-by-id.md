# get onramp transactions by id

```
{
  "next_page_key": "<string>",
  "total_count": 123,
  "transactions": [
    {
      "coinbase_fee": {
        "currency": "<string>",
        "value": "<string>"
      },
      "completed_at": "2023-11-07T05:31:56Z",
      "contract_address": "<string>",
      "country": "<string>",
      "created_at": "2023-11-07T05:31:56Z",
      "end_partner_name": "<string>",
      "exchange_rate": {
        "currency": "<string>",
        "value": "<string>"
      },
      "failure_reason": "FAILURE_REASON_BUY_FAILED",
      "network_fee": {
        "currency": "<string>",
        "value": "<string>"
      },
      "partner_user_ref": "<string>",
      "payment_method": "UNSPECIFIED",
      "payment_subtotal": {
        "currency": "<string>",
        "value": "<string>"
      },
      "payment_total": {
        "currency": "<string>",
        "value": "<string>"
      },
      "payment_total_usd": {
        "currency": "<string>",
        "value": "<string>"
      },
      "purchase_amount": {
        "currency": "<string>",
        "value": "<string>"
      },
      "purchase_currency": "<string>",
      "purchase_network": "<string>",
      "status": "ONRAMP_TRANSACTION_STATUS_CREATED",
      "transaction_id": "<string>",
      "tx_hash": "<string>",
      "type": "ONRAMP_TRANSACTION_TYPE_BUY_AND_SEND",
      "user_id": "<string>",
      "user_type": "USER_TYPE_AUTHED",
      "wallet_address": "<string>"
    }
  ]
}
```