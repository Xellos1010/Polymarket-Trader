# list entity activities

```
{
  "activities": [
    {
      "id": "<string>",
      "reference_id": "<string>",
      "category": "OTHER_ACTIVITY_CATEGORY",
      "type": "OTHER_ACTIVITY_TYPE",
      "secondary_type": "NO_SECONDARY_TYPE",
      "status": "OTHER_ACTIVITY_STATUS",
      "created_by": "<string>",
      "title": "<string>",
      "description": "<string>",
      "user_actions": [
        {
          "action": "OTHER_ACTION",
          "user_id": "<string>",
          "timestamp": "<string>"
        }
      ],
      "transactions_metadata": {
        "consensus": {
          "approval_deadline": "<string>",
          "has_passed_consensus": true
        }
      },
      "account_metadata": {
        "consensus": {
          "approval_deadline": "<string>",
          "has_passed_consensus": true
        }
      },
      "orders_metadata": {},
      "symbols": [
        "<string>"
      ],
      "created_at": "<string>",
      "updated_at": "<string>",
      "hierarchy_type": "HIERARCHY_TYPE_UNSPECIFIED"
    }
  ],
  "pagination": {
    "next_cursor": "<string>",
    "sort_direction": "DESC",
    "has_next": true
  }
}
```