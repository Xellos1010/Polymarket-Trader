# get cross margin overview

```
{
  "overview": {
    "control_status": "XM_CONTROL_STATUS_UNSPECIFIED",
    "call_status": "XM_ENTITY_CALL_STATUS_UNSPECIFIED",
    "margin_level": "XM_MARGIN_LEVEL_UNSPECIFIED",
    "margin_summary": {
      "margin_requirement": "10362.72",
      "account_equity": "-21542.63",
      "margin_excess_shortfall": "-31891.67",
      "consumed_credit": "22906.34",
      "xm_credit_limit": "1222322.00",
      "xm_margin_limit": "22123.00",
      "spot_equity": "-21505.91",
      "futures_equity": "-36.71",
      "risk_netting_info": {
        "nodal_margin_requirement": "9243.25",
        "portfolio_margin_requirement": "9003.67",
        "integrated_portfolio_margin_requirement": "10154.67",
        "ineligible_futures_margin_requirement": "194.36",
        "position_margin_requirement": "3376.45",
        "portfolio_margin_addon": "5627.21",
        "integrated_position_margin_requirement": "3376.45",
        "integrated_portfolio_margin_addon": "6778.21",
        "netted_futures_notional": "11510.00",
        "total_gmv_basis": "<string>",
        "ipm_cash_balance": "1002.94",
        "integrated_scenario_addon": {
          "amount": "<string>",
          "add_on_type": "MARGIN_ADD_ON_TYPE_UNSPECIFIED"
        },
        "all_integrated_scenario_addons": [
          {
            "amount": "<string>",
            "add_on_type": "MARGIN_ADD_ON_TYPE_UNSPECIFIED"
          }
        ],
        "xm_positions": [
          {
            "currency": "BTC",
            "market_price": "114531.73",
            "margin_eligible": true,
            "market_cap": "770000000000.00",
            "adv30_days": "1166623585.534257",
            "hist5d_vol": "0.010996074377389616",
            "hist30d_vol": "0.013617999336643158",
            "hist90d_vol": "0.015196480084775355",
            "margin_requirement": "0.015",
            "spot_balance": "-0.19652944",
            "spot_balance_notional": "-22508.85",
            "spot_total_position_margin": "<string>",
            "futures_balance": "-0.19652944",
            "futures_balance_notional": "-22508.85",
            "futures_total_position_margin": "<string>",
            "gmv_basis": "<string>",
            "base_requirement": "<string>",
            "liq_shorts_add_on": "<string>",
            "liq_longs_add_on": "<string>",
            "vol_shorts_add_on": "<string>",
            "vol_longs_add_on": "<string>",
            "vol5days_add_on": "<string>",
            "vol30days_add_on": "<string>",
            "vol90days_add_on": "<string>",
            "total_position_margin": "<string>"
          }
        ]
      }
    },
    "active_margin_calls": [
      {
        "margin_call_id": "63a2577a-930d-413b-81e4-9e77765da8f9",
        "currency": "USD",
        "initial_notional_amount": "32083.26",
        "outstanding_notional_amount": "32083.26",
        "margin_call_type": "XM_CALL_TYPE_UNSPECIFIED",
        "margin_call_status": "XM_CALL_STATUS_UNSPECIFIED",
        "called_with_margin_level": "XM_MARGIN_LEVEL_UNSPECIFIED",
        "called_with_margin_summary": {
          "margin_requirement": "10362.72",
          "account_equity": "-21542.63",
          "margin_excess_shortfall": "-31891.67",
          "consumed_credit": "22906.34",
          "xm_credit_limit": "1222322.00",
          "xm_margin_limit": "22123.00",
          "spot_equity": "-21505.91",
          "futures_equity": "-36.71",
          "risk_netting_info": {
            "nodal_margin_requirement": "9243.25",
            "portfolio_margin_requirement": "9003.67",
            "integrated_portfolio_margin_requirement": "10154.67",
            "ineligible_futures_margin_requirement": "194.36",
            "position_margin_requirement": "3376.45",
            "portfolio_margin_addon": "5627.21",
            "integrated_position_margin_requirement": "3376.45",
            "integrated_portfolio_margin_addon": "6778.21",
            "netted_futures_notional": "11510.00",
            "total_gmv_basis": "<string>",
            "ipm_cash_balance": "1002.94",
            "integrated_scenario_addon": {
              "amount": "<string>",
              "add_on_type": "MARGIN_ADD_ON_TYPE_UNSPECIFIED"
            },
            "all_integrated_scenario_addons": [
              {
                "amount": "<string>",
                "add_on_type": "MARGIN_ADD_ON_TYPE_UNSPECIFIED"
              }
            ],
            "xm_positions": [
              {
                "currency": "BTC",
                "market_price": "114531.73",
                "margin_eligible": true,
                "market_cap": "770000000000.00",
                "adv30_days": "1166623585.534257",
                "hist5d_vol": "0.010996074377389616",
                "hist30d_vol": "0.013617999336643158",
                "hist90d_vol": "0.015196480084775355",
                "margin_requirement": "0.015",
                "spot_balance": "-0.19652944",
                "spot_balance_notional": "-22508.85",
                "spot_total_position_margin": "<string>",
                "futures_balance": "-0.19652944",
                "futures_balance_notional": "-22508.85",
                "futures_total_position_margin": "<string>",
                "gmv_basis": "<string>",
                "base_requirement": "<string>",
                "liq_shorts_add_on": "<string>",
                "liq_longs_add_on": "<string>",
                "vol_shorts_add_on": "<string>",
                "vol_longs_add_on": "<string>",
                "vol5days_add_on": "<string>",
                "vol30days_add_on": "<string>",
                "vol90days_add_on": "<string>",
                "total_position_margin": "<string>"
              }
            ]
          }
        },
        "due_at": "2023-11-07T05:31:56Z",
        "created_at": "2023-11-07T05:31:56Z",
        "updated_at": "2023-11-07T05:31:56Z"
      }
    ],
    "active_loans": [
      {
        "loan_id": "b91a0ed6-eeec-4496-a04e-98b72b33c2b4",
        "loan_party": "XM_PARTY_UNSPECIFIED",
        "principal_currency": "BTC",
        "principal_currency_market_price": "114531.73",
        "initial_principal_amount": "0.2",
        "outstanding_principal_amount": "0.2",
        "created_at": "2023-11-07T05:31:56Z",
        "updated_at": "2023-11-07T05:31:56Z"
      }
    ]
  }
}
```