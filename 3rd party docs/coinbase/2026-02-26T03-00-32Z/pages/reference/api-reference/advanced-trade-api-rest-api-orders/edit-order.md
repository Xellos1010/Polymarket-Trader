# edit order

```
curl --request POST \
  --url https://api.coinbase.com/api/v3/brokerage/orders/edit \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "order_id": "<string>",
  "price": "19000.00",
  "size": "0.001",
  "attached_order_configuration": {
    "market_market_ioc": {
      "quote_size": "10.00",
      "base_size": "0.001"
    },
    "market_market_fok": {
      "quote_size": "10.00",
      "base_size": "0.001"
    },
    "sor_limit_ioc": {
      "quote_size": "10.00",
      "base_size": "0.001",
      "limit_price": "10000.00"
    },
    "limit_limit_gtc": {
      "quote_size": "10.00",
      "base_size": "0.001",
      "limit_price": "10000.00",
      "post_only": false
    },
    "limit_limit_gtd": {
      "quote_size": "10.00",
      "base_size": "0.001",
      "limit_price": "10000.00",
      "end_time": "2021-05-31T09:59:59.000Z",
      "post_only": false
    },
    "limit_limit_fok": {
      "quote_size": "10.00",
      "base_size": "0.001",
      "limit_price": "10000.00"
    },
    "twap_limit_gtd": {
      "quote_size": "10.00",
      "base_size": "0.001",
      "start_time": "2021-05-31T07:59:59.000Z",
      "end_time": "2021-05-31T09:59:59.000Z",
      "limit_price": "10000.00",
      "number_buckets": "5",
      "bucket_size": "2.00",
      "bucket_duration": "300s"
    },
    "stop_limit_stop_limit_gtc": {
      "base_size": "0.001",
      "limit_price": "10000.00",
      "stop_price": "20000.00",
      "stop_direction": "20000.00"
    },
    "stop_limit_stop_limit_gtd": {
      "base_size": 0.001,
      "limit_price": "10000.00",
      "stop_price": "20000.00",
      "end_time": "2021-05-31T09:59:59.000Z",
      "stop_direction": "20000.00"
    },
    "trigger_bracket_gtc": {
      "base_size": 0.001,
      "limit_price": "10000.00",
      "stop_trigger_price": "20000.00"
    },
    "trigger_bracket_gtd": {
      "base_size": 0.001,
      "limit_price": "10000.00",
      "stop_trigger_price": "20000.00",
      "end_time": "2021-05-31T09:59:59.000Z"
    },
    "scaled_limit_gtc": {
      "orders": [
        {
          "quote_size": "10.00",
          "base_size": "0.001",
          "limit_price": "10000.00",
          "post_only": false
        }
      ],
      "quote_size": "<string>",
      "base_size": "<string>",
      "num_orders": 123,
      "min_price": "<string>",
      "max_price": "<string>",
      "price_distribution": "FLAT",
      "size_distribution": "UNKNOWN_DISTRIBUTION",
      "size_diff": "<string>",
      "size_ratio": "<string>"
    }
  },
  "cancel_attached_order": "true",
  "stop_price": "17000.00"
}
'
```