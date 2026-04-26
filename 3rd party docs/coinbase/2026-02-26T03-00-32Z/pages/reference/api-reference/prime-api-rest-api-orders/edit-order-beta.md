# edit order beta

```
curl --request PUT \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/orders/{order_id}/edit \
  --header 'Content-Type: application/json' \
  --data '
{
  "orig_client_order_id": "<string>",
  "client_order_id": "<string>",
  "product_id": "<string>",
  "base_quantity": "<string>",
  "quote_value": "<string>",
  "limit_price": "<string>",
  "expiry_time": "2023-11-07T05:31:56Z",
  "display_quote_size": "<string>",
  "display_base_size": "<string>",
  "stop_price": "<string>"
}
'
```