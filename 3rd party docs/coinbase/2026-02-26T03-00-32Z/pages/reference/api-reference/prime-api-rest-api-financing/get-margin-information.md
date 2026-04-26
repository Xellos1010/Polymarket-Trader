# get margin information

```
{
  "margin_information": {
    "margin_call_records": [
      {
        "margin_call_id": "e8bbed13-fa33-41de-86d5-4335d8f08122",
        "initial_notional_amount": "123.45",
        "outstanding_notional_amount": "122.45",
        "created_at": "2023-01-01T00:00:00Z5",
        "due_at": "2023-01-01T00:00:00.000Z"
      }
    ],
    "margin_summary": {
      "entity_id": "e8bbed13-fa33-41de-86d5-4335d8f08166",
      "margin_equity": "2539231.5903575355",
      "margin_requirement": "15131.725279884091",
      "excess_deficit": "2524099.865077651",
      "pm_credit_consumed": "10000",
      "tf_credit_limit": "2000000",
      "tf_credit_consumed": "20000",
      "tf_adjusted_asset_value": "40000",
      "tf_adjusted_liability_value": "40000",
      "tf_adjusted_credit_consumed": "30000",
      "tf_adjusted_equity": "30000",
      "frozen": false,
      "frozen_reason": "<string>",
      "tf_enabled": true,
      "pm_enabled": true,
      "market_rates": [
        {
          "symbol": "BTC",
          "rate": "27,123"
        }
      ],
      "asset_balances": [
        {
          "portfolio_id": "e8bbed13-fa33-41de-86d5-4335d8f08166",
          "symbol": "BTC",
          "amount": "4000",
          "notional_amount": "4000",
          "conversion_rate": "1000"
        }
      ],
      "tf_loans": [
        {
          "portfolio_id": "e8bbed13-fa33-41de-86d5-4335d8f08166",
          "symbol": "BTC",
          "amount": "150000",
          "notional_amount": "250000",
          "due_date": "1000"
        }
      ],
      "pm_loans": [
        {
          "portfolio_id": "e8bbed13-fa33-41de-86d5-4335d8f08166",
          "symbol": "BTC",
          "amount": "150000",
          "notional_amount": "250000",
          "due_date": "1000"
        }
      ],
      "short_collateral": [
        {
          "portfolio_id": "e8bbed13-fa33-41de-86d5-4335d8f08166",
          "symbol": "BTC",
          "amount": "150000",
          "notional_amount": "250000",
          "due_date": "1000"
        }
      ],
      "gross_market_value": "<string>",
      "net_market_value": "<string>",
      "long_market_value": "<string>",
      "non_marginable_long_market_value": "<string>",
      "short_market_value": "<string>",
      "gross_leverage": "<string>",
      "net_exposure": "<string>",
      "portfolio_stress_triggered": {
        "amount": "<string>",
        "add_on_type": "MARGIN_ADD_ON_TYPE_UNSPECIFIED"
      },
      "pm_asset_info": [
        {
          "symbol": "<string>",
          "amount": "<string>",
          "price": "<string>",
          "notional_amount": "<string>",
          "asset_tier": "<string>",
          "margin_eligible": true,
          "base_margin_requirement": "<string>",
          "base_margin_requirement_notional": "<string>",
          "adv_30d": "<string>",
          "hist_5d_vol": "<string>",
          "hist_30d_vol": "<string>",
          "hist_90d_vol": "<string>",
          "volatility_addon": "<string>",
          "liquidity_addon": "<string>",
          "total_position_margin": "<string>",
          "short_nominal": "<string>",
          "long_nominal": "<string>"
        }
      ],
      "pm_credit_limit": "1000000",
      "pm_margin_limit": "200000",
      "pm_margin_consumed": "5000"
    }
  }
}
```