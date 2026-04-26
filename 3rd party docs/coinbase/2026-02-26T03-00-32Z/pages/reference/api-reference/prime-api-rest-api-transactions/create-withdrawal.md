# create withdrawal

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/wallets/{wallet_id}/withdrawals \
  --header 'Content-Type: application/json' \
  --data '
{
  "amount": "<string>",
  "destination_type": "DESTINATION_PAYMENT_METHOD",
  "idempotency_key": "<string>",
  "currency_symbol": "<string>",
  "payment_method": {
    "payment_method_id": "<string>"
  },
  "blockchain_address": {
    "address": "<string>",
    "account_identifier": "<string>",
    "network": {
      "id": "<string>",
      "type": "<string>"
    }
  },
  "counterparty": {
    "counterparty_id": "<string>"
  },
  "travel_rule_data": {
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
    "is_self": true,
    "is_intermediary": true,
    "opt_out_of_ownership_verification": true,
    "attest_verified_wallet_ownership": true
  }
}
'
```