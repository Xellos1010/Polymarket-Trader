# currencies

## Table of Endpoints

Name

Method

Endpoint

Scope

[Get Fiat Currencies](#get-fiat-currencies)

GET

`/v2/currencies`

N/A

[Get Cryptocurrencies](#get-cryptocurrencies)

GET

`/v2/currencies/crypto`

N/A

## Get Fiat Currencies

Lists known fiat currencies. Currency codes conform to the ISO 4217 standard where possible. Currencies with no representation in ISO 4217 may use a custom code. **This endpoint doesn’t require authentication.**

### HTTP Request

`GET https://api.coinbase.com/v2/currencies`

### Scopes

-   *No permission required*

### Examples

#### Request

#### Response

```
{
  "data": [
    {
      "id": "AED",
      "name": "United Arab Emirates Dirham",
      "min_size": "0.01000000"
    },
    {
      "id": "AFN",
      "name": "Afghan Afghani",
      "min_size": "0.01000000"
    },
    {
      "id": "ALL",
      "name": "Albanian Lek",
      "min_size": "0.01000000"
    },
    {
      "id": "AMD",
      "name": "Armenian Dram",
      "min_size": "0.01000000"
    }
  ],
  ...
}

```

## Get Cryptocurrencies

Lists known cryptocurrencies. **This endpoint doesn’t require authentication.**

### HTTP Request

`GET https://api.coinbase.com/v2/currencies/crypto`

### Scopes

-   *No permission required*

### Examples

#### Request

#### Response

```
[
  {
    "code": "BTC",
    "name": "Bitcoin",
    "color": "#F7931A",
    "sort_index": 100,
    "exponent": 8,
    "type": "crypto",
    "address_regex": "^([13][a-km-zA-HJ-NP-Z1-9]{25,34})|^(bc1[qzry9x8gf2tvdw0s3jn54khce6mua7l]([qpzry9x8gf2tvdw0s3jn54khce6mua7l]{38}|[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{58}))$",
    "asset_id": "5b71fc48-3dd3-540c-809b-f8c94d0e68b5"
  },
  {
    "code": "ETH",
    "name": "Ethereum",
    "color": "#627EEA",
    "sort_index": 102,
    "exponent": 8,
    "type": "crypto",
    "address_regex": "^(?:0x)?[0-9a-fA-F]{40}$",
    "asset_id": "d85dce9b-5b73-5c3c-8978-522ce1d1c1b4"
  },
  {
    "code": "ETH2",
    "name": "Ethereum 2",
    "color": "#8E76FF",
    "sort_index": 161,
    "exponent": 8,
    "type": "crypto",
    "address_regex": "^(?:0x)?[0-9a-fA-F]{40}$",
    "asset_id": "3bec5bf3-507a-51ba-8e41-dc953b1a5c4d"
  },
  {
    "code": "ETC",
    "name": "Ethereum Classic",
    "color": "#59D4AF",
    "sort_index": 103,
    "exponent": 8,
    "type": "crypto",
    "address_regex": "^(?:0x)?[0-9a-fA-F]{40}$",
    "asset_id": "c16df856-0345-5358-8a70-2a78c804e61f"
  }
]

```