# get lending overview xm

```
{
  "body": {
    "overview": {
      "open_loan_value": "<string>",
      "available_to_borrow": "<string>",
      "withdrawal_restricted": true,
      "credit_limit_value": "<string>",
      "available_credit_value": "<string>",
      "pending_loan_value": "<string>",
      "margin_requirement": "<string>",
      "account_equity": "<string>",
      "margin_excess_shortfall": "<string>",
      "consumed_credit": "<string>"
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
}
```