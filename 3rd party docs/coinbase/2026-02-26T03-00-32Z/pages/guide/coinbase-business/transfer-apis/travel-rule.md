# travel rule

The [Travel Rule](https://www.coinbase.com/travelrule) requires financial institutions, including custodial cryptocurrency exchanges, to share basic information about their customers when sending funds over a certain amount. Required Travel Rule fields differ by region. These requirements are determined based on which Coinbase entity the user has signed the service agreement for.

### Arguments

Parameter

Type

Description

`beneficiary_wallet_type`

string

Whether the transfer is to a self hosted wallet or to an exchange: \[`WALLET_TYPE_SELF_HOSTED`, `WALLET_TYPE_EXCHANGE`\]

`is_self`

string

Whether the transfer is to a wallet owned by the user: \[`IS_SELF_FALSE`, `IS_SELF_TRUE`\]

`beneficiary_name`

string

Full name of the beneficiary, e.g. “John Smith”

`beneficiary_address`

object

Beneficiary postal address: `{"address1" : string, "address2": string, "address3": string, "city": string, "state": string, "country": ISO 3166-1 alpha-2 country code string, "postal_code": string}`

`beneficiary_financial_institution`

string

[Beneficiary Financial Institution or Virtual Asset Provider](https://developer.chrome.com/coinbase-app/transfer-apis/vasps): `009BBC11-03FB-4224-8CC7-CCE7CCDE4706`

`transfer_purpose`

string

Purpose of transfer, e.g. ‘Rent’.

Note: `beneficiary_country` refers to `country` in `beneficiary_address` and is in `ISO 3166-1 alpha-2 country code` format. Visit this [link](https://developer.chrome.com/coinbase-app/transfer-apis/vasps) to find the values of `beneficiary_financial_institution` Example Travel Rule data in a Transaction:

```
{
  "type": "send",
  "to": "1AUJ8z5RuHRTqD1eikyfUUetzGmdWLGkpT",
  "amount": "0.1",
  "currency": "BTC",
  "idem": "9316dd16-0c05",
  "network": "bitcoin",
  "travel_rule_data": {
    "beneficiary_wallet_type": "WALLET_TYPE_EXCHANGE",
    "is_self": "IS_SELF_TRUE",
    "beneficiary_name": "San Francisco City Hall",
    "beneficiary_address": {
      "address1": "1 Dr Carlton B Goodlett Pl",
      "address2": "Unit 201",
      "address3": "",
      "city": "San Francisco",
      "state": "CA",
      "postal_code": "94102",
      "country": "US"
    },
    "beneficiary_financial_institution": "009BBC11-03FB-4224-8CC7-CCE7CCDE4706",
    "transfer_purpose": "Test"
  }
}

```

Example Travel Rule Data:

```
{
  "beneficiary_wallet_type": "WALLET_TYPE_EXCHANGE",
  "is_self": "IS_SELF_TRUE",
  "beneficiary_name": "San Francisco City Hall",
  "beneficiary_address": {
    "address1": "1 Dr Carlton B Goodlett Pl",
    "address2": "Unit 201",
    "address3": "",
    "city": "San Francisco",
    "state": "CA",
    "postal_code": "94102",
    "country": "US"
  },
  "beneficiary_financial_institution": "009BBC11-03FB-4224-8CC7-CCE7CCDE4706",
  "transfer_purpose": "Test"
}

```

## United States (US)

Currently, populating the `travel_rule_data` field is not required.

## Canada (CA)

The required fields are `beneficiary_wallet_type` Example with the `WALLET_TYPE_SELF_HOSTED`:

```
{
  "beneficiary_wallet_type": "WALLET_TYPE_SELF_HOSTED"
}

```

If the `beneficiary_wallet_type` is equal to `WALLET_TYPE_EXCHANGE` and the transaction value is greater than or equal to `999` CAD, then `BENEFICIARY_NAME` and `BENEFICIARY_ADDRESS` fields are also required: Example:

```
{
  "beneficiary_wallet_type": "WALLET_TYPE_EXCHANGE",
  "beneficiary_name": "Satoshi Nakamoto",
  "beneficiary_address": {
    "address1": "1 Dr Carlton B Goodlett Pl",
    "city": "San Francisco",
    "state": "CA",
    "postal_code": "94102",
    "country": "US"
  }
}

```

## Great Britain (GB)

The required fields are `is_self`, `beneficiary_name`, `beneficiary_country` and `beneficiary_wallet_type`

```
{
  "is_self": "IS_SELF_TRUE",
  "beneficiary_name": "Satoshi Nakamoto",
  "beneficiary_address": { "country": "US" },
  "beneficiary_wallet_type": "WALLET_TYPE_SELF_HOSTED"
}

```

If `beneficiary_wallet_type` is equal to `WALLET_TYPE_EXCHANGE`, then [`beneficiary_financial_institution`](https://developer.chrome.com/coinbase-app/transfer-apis/vasps) field is needed. This can field can be found on the Travel Rule VASP lists which contains both the name and id. Example:

```
{
  "is_self": "IS_SELF_TRUE",
  "beneficiary_name": "Satoshi Nakamoto",
  "beneficiary_address": { "country": "US" },
  "beneficiary_wallet_type": "WALLET_TYPE_SELF_EXCHANGE",
  "beneficiary_financial_institution": "009BBC11-03FB-4224-8CC7-CCE7CCDE4706"
}

```

## Bermuda (BM)

The required fields are `beneficiary_name`, `beneficiary_wallet_type` and `is_self` Example:

```
{
  "is_self": "IS_SELF_TRUE",
  "beneficiary_name": "Satoshi Nakamoto",
  "beneficiary_wallet_type": "WALLET_TYPE_SELF_EXCHANGE"
}

```

## Singapore, Hong Kong, Philippines (SG/HK/PH)

The required fields are `beneficiary_name`, `beneficiary_wallet_type`, `beneficiary_country`, and `is_self`. If the `beneficiary_wallet_type` is `WALLET_TYPE_SELF_HOSTED` A Satoshi Test is required to verify ownership of a wallet. This is done through the Coinbase Mobile app. Example:

```
{
  "is_self": "IS_SELF_TRUE",
  "beneficiary_name": "Satoshi Nakamoto",
  "beneficiary_address": {
    "address1": "1 Dr Carlton B Goodlett Pl",
    "city": "San Francisco",
    "state": "CA",
    "postal_code": "94102",
    "country": "US"
  },
  "beneficiary_wallet_type": "WALLET_TYPE_SELF_HOSTED"
}

```

If the `beneficiary_wallet_type` is `WALLET_TYPE_EXCHANGE`, then [`beneficiary_financial_institution`](https://developer.chrome.com/coinbase-app/transfer-apis/vasps) field is needed. Example:

```
{
  "is_self": "IS_SELF_TRUE",
  "beneficiary_name": "Satoshi Nakamoto",
  "beneficiary_address": {
    "address1": "1 Dr Carlton B Goodlett Pl",
    "city": "San Francisco",
    "state": "CA",
    "postal_code": "94102",
    "country": "US"
  },
  "beneficiary_wallet_type": "WALLET_TYPE_SELF_EXCHANGE",
  "beneficiary_financial_institution": "009BBC11-03FB-4224-8CC7-CCE7CCDE4706"
}

```

## European Union (EU)

This includes:

-   Austria (AT)
-   Belgium (BE)
-   Bulgaria (BG)
-   Cyprus (CY)
-   Czech Republic (CZ)
-   Germany (DE)
-   Denmark (DK)
-   Estonia (EE)
-   Spain (ES)
-   Finland (FI)
-   France (FR)
-   Greece (GR)
-   Croatia (HR)
-   Hungary (HU)
-   Ireland (IE)
-   Iceland (IS)
-   Italy (IT)
-   Liechtenstein (LI)
-   Lithuania (LT)
-   Luxembourg (LU)
-   Latvia (LV)
-   Malta (MT)
-   Netherlands (NL)
-   Norway (NO)
-   Poland (PL)
-   Portugal (PT)
-   Romania (RO)
-   Sweden (SE)
-   Slovenia (SI)
-   Slovakia (SK)

The required fields are `beneficiary_name`, `beneficiary_wallet_type`, `beneficiary_country`, and `is_self`. If the `beneficiary_wallet_type` is `WALLET_TYPE_SELF_HOSTED` and amount is greater than 1000 EURO, a Satoshi Test is required to verify ownership of a wallet. This is done through the Coinbase Mobile app. Example:

```
{
  "is_self": "IS_SELF_TRUE",
  "beneficiary_name": "Satoshi Nakamoto",
  "beneficiary_address": {
    "address1": "1 Dr Carlton B Goodlett Pl",
    "city": "San Francisco",
    "state": "CA",
    "postal_code": "94102",
    "country": "US"
  },
  "beneficiary_wallet_type": "WALLET_TYPE_SELF_HOSTED"
}

```

If the `beneficiary_wallet_type` is `WALLET_TYPE_EXCHANGE`, then [`beneficiary_financial_institution`](https://developer.chrome.com/coinbase-app/transfer-apis/vasps) field is needed. Example:

```
{
  "is_self": "IS_SELF_TRUE",
  "beneficiary_name": "Satoshi Nakamoto",
  "beneficiary_address": {
    "address1": "1 Dr Carlton B Goodlett Pl",
    "city": "San Francisco",
    "state": "CA",
    "postal_code": "94102",
    "country": "US"
  },
  "beneficiary_wallet_type": "WALLET_TYPE_SELF_EXCHANGE",
  "beneficiary_financial_institution": "009BBC11-03FB-4224-8CC7-CCE7CCDE4706"
}

```

## Non-Listed Jurisdictions & Countries

If your jurisdiction or country is not listed, then there are no additional `travel_rule_data` requirements needed, and you do not have to populate it.