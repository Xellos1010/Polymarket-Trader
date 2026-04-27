# get transaction summary

```
{
  "total_fees": 25,
  "fee_tier": {
    "pricing_tier": "<$10k",
    "taker_fee_rate": "0.0010",
    "maker_fee_rate": "0.0020",
    "aop_from": "0",
    "aop_to": "10000",
    "volume_types_and_range": [
      {
        "volume_types": [
          "VOLUME_TYPE_SPOT",
          "VOLUME_TYPE_US_DERIVATIVES"
        ],
        "vol_from": "0",
        "vol_to": "50000"
      }
    ]
  },
  "margin_rate": 0.5,
  "goods_and_services_tax": {
    "rate": "<string>",
    "type": "INCLUSIVE"
  },
  "advanced_trade_only_volume": 1000,
  "advanced_trade_only_fees": 25,
  "coinbase_pro_volume": 1000,
  "coinbase_pro_fees": 25,
  "total_balance": "1000",
  "volume_breakdown": [
    {
      "volume_type": "VOLUME_TYPE_SPOT",
      "volume": 1000
    }
  ],
  "has_cost_plus_commission": false
}
```