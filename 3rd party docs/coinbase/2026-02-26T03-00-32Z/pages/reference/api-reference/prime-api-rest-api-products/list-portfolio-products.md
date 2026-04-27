# list portfolio products

```
{
  "products": [
    {
      "id": "BTC-USD",
      "base_increment": "1",
      "quote_increment": "1",
      "base_min_size": "100",
      "quote_min_size": "100",
      "base_max_size": "1000",
      "quote_max_size": "1000",
      "permissions": "PRODUCT_PERMISSION_READ",
      "price_increment": "0.01",
      "rfq_product_details": {
        "tradable": true,
        "min_notional_size": "<string>",
        "max_notional_size": "<string>",
        "min_base_size": "<string>",
        "max_base_size": "<string>",
        "min_quote_size": "<string>",
        "max_quote_size": "<string>"
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