# list order edit history

#### Path Parameters

#### Response

edit\_history

The list of edits to the order · object\[\]

required

The history of order edits

Example:

```
[  
  {  
    "price": "100",  
    "base_quantity": "1",  
    "quote_value": "100",  
    "display_base_size": "1.00",  
    "display_quote_size": "1.00",  
    "stop_price": "120",  
    "expiry_time": "2021-05-31T10:59:59.000Z",  
    "accept_time": "2021-06-20T10:59:59.000Z",  
    "client_order_id": "123"  
  }  
]
```

order\_edit\_history

LimitOrderEdit represents an order edit that is accepted · object\[\]

The history of order edits (deprecated: use edit\_history instead)