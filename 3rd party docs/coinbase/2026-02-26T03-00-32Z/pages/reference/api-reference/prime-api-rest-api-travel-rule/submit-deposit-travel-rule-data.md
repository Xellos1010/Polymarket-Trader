# submit deposit travel rule data

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/transactions/{transaction_id}/travel_rule/deposit \
  --header 'Content-Type: application/json' \
  --data '
{
  "originator": {
    "name": "<string>",
    "natural_person_name": {
      "first_name": "<string>",
      "middle_name": "<string>",
      "last_name": "<string>"
    },
    "address": {
      "address_1": "<string>",
      "address_2": "<string>",
      "address_3": "<string>",
      "city": "<string>",
      "state": "<string>",
      "country_code": "<string>",
      "postal_code": "<string>"
    },
    "wallet_type": "TRAVEL_RULE_WALLET_TYPE_UNSPECIFIED",
    "vasp_id": "<string>",
    "vasp_name": "<string>",
    "personal_id": "<string>",
    "date_of_birth": {
      "year": 123,
      "month": 123,
      "day": 123
    },
    "telephone_number": "<string>",
    "account_id": "<string>"
  },
  "beneficiary": {
    "name": "<string>",
    "natural_person_name": {
      "first_name": "<string>",
      "middle_name": "<string>",
      "last_name": "<string>"
    },
    "address": {
      "address_1": "<string>",
      "address_2": "<string>",
      "address_3": "<string>",
      "city": "<string>",
      "state": "<string>",
      "country_code": "<string>",
      "postal_code": "<string>"
    },
    "wallet_type": "TRAVEL_RULE_WALLET_TYPE_UNSPECIFIED",
    "vasp_id": "<string>",
    "vasp_name": "<string>",
    "personal_id": "<string>",
    "date_of_birth": {
      "year": 123,
      "month": 123,
      "day": 123
    },
    "telephone_number": "<string>",
    "account_id": "<string>"
  },
  "is_self": true,
  "opt_out_of_ownership_verification": true
}
'
```