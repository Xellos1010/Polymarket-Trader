# create order

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/order \
  --header 'Content-Type: application/json' \
  --data '
{
  "product_id": "<string>",
  "side": "BUY",
  "client_order_id": "<string>",
  "type": "MARKET",
  "base_quantity": "<string>",
  "quote_value": "<string>",
  "limit_price": "<string>",
  "start_time": "2023-11-07T05:31:56Z",
  "expiry_time": "2023-11-07T05:31:56Z",
  "time_in_force": "GOOD_UNTIL_DATE_TIME",
  "stp_id": "<string>",
  "display_quote_size": "<string>",
  "display_base_size": "<string>",
  "is_raise_exact": true,
  "historical_pov": "<string>",
  "stop_price": "<string>",
  "settl_currency": "<string>",
  "post_only": true,
  "peg_offset_type": "PEG_OFFSET_TYPE_PRICE",
  "offset": "<string>",
  "wig_level": "<string>"
}
'
```