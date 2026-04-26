# withdraw to crypto address

```
curl --request POST \
  --url https://api.exchange.coinbase.com/withdrawals/crypto \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "amount": "<string>",
  "currency": "<string>",
  "crypto_address": "<string>",
  "profile_id": "<string>",
  "destination_tag": "<string>",
  "no_destination_tag": true,
  "nonce": 123,
  "network": "<string>",
  "add_network_fee_to_total": true,
  "is_intermediary": false,
  "travel_rule_data": {
    "originator_name": "<string>",
    "originator_natural_name": {
      "first_name": "<string>",
      "last_name": "<string>"
    },
    "originator_address": {
      "address_1": "<string>",
      "address_2": "<string>",
      "address_3": "<string>",
      "city": "<string>",
      "state": "<string>",
      "country": "<string>",
      "postal_code": "<string>"
    },
    "originator_telephone_number": "<string>",
    "originator_account": "<string>",
    "originator_account_location_country_code": "<string>",
    "originator_date_of_birth": {
      "year": 123,
      "month": 123,
      "day": 123
    },
    "originator_account_number": "<string>",
    "transfer_purpose": "<string>",
    "beneficiary_name": "<string>",
    "beneficiary_address": {
      "address_1": "<string>",
      "address_2": "<string>",
      "address_3": "<string>",
      "city": "<string>",
      "state": "<string>",
      "country": "<string>",
      "postal_code": "<string>"
    },
    "beneficiary_telephone_number": "<string>",
    "beneficiary_account_location": "<string>",
    "beneficiary_date_of_birth": {
      "year": 123,
      "month": 123,
      "day": 123
    },
    "beneficiary_financial_institution": "<string>",
    "is_self": true,
    "originator_wallet_address": "<string>",
    "originating_vasp_for_intermediary": {
      "attest_verified_wallet_ownership": true
    }
  }
}
'
```