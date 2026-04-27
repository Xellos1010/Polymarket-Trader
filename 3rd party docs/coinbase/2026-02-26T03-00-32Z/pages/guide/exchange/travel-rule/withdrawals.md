# withdrawals

The Travel Rule requires financial institutions, including custodial cryptocurrency exchanges, to share basic information about their customers when sending funds over a certain amount. VASPs (Virtual Asset Service Providers) like Coinbase that are part of the TRUST (Travel Rule Universal Solution Technology) consortium use the [TRUST solution](https://www.coinbase.com/travelrule) when sharing PII (Personally Identifiable Information) in order to satisfy the Travel Rule data requirements. The [Withdraw to crypto address](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/withdraw-to-crypto-address) endpoint supports the Travel Rule as follows:

Coinbase as a VASP

Depending on the jurisdiction, you may be required to provide data related to the beneficiary of the withdrawal.Users in travel-rule jurisdictions can only withdraw to addresses that have been added to their address-book. In such cases, the `travel_rule_data` is obtained from the address-book. Please note that [`post /address-book`](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/withdraw-to-crypto-address)) has fields to support this.Example request:

```
curl -L -X POST 'https://api.exchange.coinbase.com/withdrawals/crypto' \
-H "Content-Type: application/json" \
-d "@data.json"

```

`data.json` content:

```
{
  "amount": "1.0",
  "currency": "BTC",
  "crypto_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
}

```

Coinbase as an intermediary VASP

Error responses for missing Travel Rule data

When the required Travel Rule data has not been provided for a given jurisdiction, an error response will be received, such as the following (HTTP status code 400):

```
{
  "message": "missing fields to satisfy travel rule requirements",
  "missing_fields": ["beneficiary_name", "beneficiary_address", "originator_name"]
}

```