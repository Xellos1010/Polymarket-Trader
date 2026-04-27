# get lending overview

```
{
  "overview": {
    "open_loan_value": "<string>",
    "collateral_value": "<string>",
    "collateralization_percentage": "<string>",
    "available_to_borrow": "<string>",
    "available_per_asset": {},
    "withdrawal_restricted": true,
    "credit_limit_value": "<string>",
    "available_credit_value": "<string>",
    "collateralization_percentage_open_only": "<string>",
    "pending_loan_value": "<string>",
    "initial_margin_percentage": "<string>",
    "minimum_margin_percentage": "<string>",
    "unlock_margin_percentage": "<string>"
  },
  "loans": [
    {
      "id": "<string>",
      "currency": "<string>",
      "principal_amount": "<string>",
      "outstanding_principal_amount": "<string>",
      "interest_rate": "<string>",
      "interest_currency": "<string>",
      "status": "loan_pending",
      "effective_at": "2023-11-07T05:31:56Z",
      "closed_at": "2023-11-07T05:31:56Z",
      "term_start_date": "2023-11-07T05:31:56Z",
      "term_end_date": "2023-11-07T05:31:56Z"
    }
  ]
}
```