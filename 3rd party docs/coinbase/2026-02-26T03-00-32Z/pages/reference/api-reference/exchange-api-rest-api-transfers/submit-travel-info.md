# submit travel info

```
curl --request POST \
  --url https://api.exchange.coinbase.com/transfers/{transfer_id}/travel-rules \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "transfer_id": "<string>",
  "originator_name": "<string>",
  "originator_country": "<string>",
  "vasp_id": "<string>",
  "vasp_country": "<string>",
  "lei_number": "<string>",
  "business_address": {
    "address_1": "<string>",
    "address_2": "<string>",
    "address_3": "<string>",
    "city": "<string>",
    "state": "<string>",
    "country": "<string>",
    "postal_code": "<string>"
  },
  "wallet_type": "UNKNOWN_WALLET_TYPE",
  "is_self_certified": true
}
'
```