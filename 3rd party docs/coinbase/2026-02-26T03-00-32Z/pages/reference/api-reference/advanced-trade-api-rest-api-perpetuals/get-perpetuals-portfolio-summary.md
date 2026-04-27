# get perpetuals portfolio summary

```
{
  "portfolios": [
    {
      "portfolio_uuid": "<string>",
      "collateral": "<string>",
      "position_notional": "<string>",
      "open_position_notional": "<string>",
      "pending_fees": "<string>",
      "borrow": "<string>",
      "accrued_interest": "<string>",
      "rolling_debt": "<string>",
      "portfolio_initial_margin": "<string>",
      "portfolio_im_notional": {
        "value": "<string>",
        "currency": "<string>"
      },
      "portfolio_maintenance_margin": "<string>",
      "portfolio_mm_notional": {
        "value": "<string>",
        "currency": "<string>"
      },
      "liquidation_percentage": "<string>",
      "liquidation_buffer": "<string>",
      "margin_type": "MARGIN_TYPE_UNSPECIFIED",
      "margin_flags": "PORTFOLIO_MARGIN_FLAGS_UNSPECIFIED",
      "liquidation_status": "PORTFOLIO_LIQUIDATION_STATUS_UNSPECIFIED",
      "unrealized_pnl": {
        "value": "<string>",
        "currency": "<string>"
      },
      "total_balance": {
        "value": "<string>",
        "currency": "<string>"
      }
    }
  ],
  "summary": {
    "unrealized_pnl": {
      "value": "<string>",
      "currency": "<string>"
    },
    "buying_power": {
      "value": "<string>",
      "currency": "<string>"
    },
    "total_balance": {
      "value": "<string>",
      "currency": "<string>"
    },
    "max_withdrawal_amount": {
      "value": "<string>",
      "currency": "<string>"
    }
  }
}
```