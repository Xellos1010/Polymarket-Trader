# get all payment methods

```
[
  {
    "id": "cbdd9f28-34e7-5152-b1dc-d657bf8df858",
    "type": "fiat_account",
    "name": "Cash (USD)",
    "currency": "USD",
    "primary_buy": true,
    "primary_sell": true,
    "instant_buy": true,
    "instant_sell": true,
    "created_at": "2019-06-04T21:24:32.000Z",
    "updated_at": "2019-06-04T21:24:32.000Z",
    "resource": "payment_method",
    "resource_path": "/v2/payment-methods/cbdd9f28-34e7-5152-b1dc-d657bf8df858",
    "limits": {
      "type": "fiat_account",
      "name": "Coinbase Account"
    },
    "allow_buy": true,
    "allow_sell": true,
    "allow_deposit": false,
    "allow_withdraw": false,
    "fiat_account": {
      "id": "2b760113-fbba-5600-ac74-36482c130768",
      "resource": "account",
      "resource_path": "/v2/accounts/2b760113-fbba-5600-ac74-36482c130768"
    },
    "verified": true,
    "picker_data": {
      "symbol": "fiat_account",
      "balance": {
        "amount": "1.00",
        "currency": "USD"
      }
    },
    "hold_business_days": 0,
    "hold_days": 0
  }
]
```