# list invoices

```
{
  "invoices": [
    {
      "id": "<string>",
      "billing_month": 123,
      "billing_year": 123,
      "due_date": "<string>",
      "invoice_number": "<string>",
      "state": "INVOICE_STATE_UNSPECIFIED",
      "usd_amount_paid": 123,
      "usd_amount_owed": 123,
      "invoice_items": [
        {
          "description": "<string>",
          "currency_symbol": "<string>",
          "invoice_type": "INVOICE_TYPE_UNSPECIFIED",
          "rate": 123,
          "quantity": 123,
          "price": 123,
          "average_auc": 123,
          "total": 123
        }
      ]
    }
  ]
}
```