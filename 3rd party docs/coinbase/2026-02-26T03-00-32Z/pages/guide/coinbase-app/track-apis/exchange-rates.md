# exchange rates

## Table of Endpoints

Name

Method

Endpoint

Scope

[Get Exchange Rates](#get-exchange-rates)

GET

`/v2/exchange-rates`

N/A

Get current exchange rates. Default base currency is `USD` but it can be defined as any supported currency (see `Currencies` endpoint). Returned rates will define the exchange rate for one unit of the base currency. **This endpoint doesn’t require authentication.**

### HTTP Request

`GET https://api.coinbase.com/v2/exchange-rates`

### Scopes

-   *No permission required*

### Arguments

Parameter

Type

Required

Description

currency

string

Optional

Base currency (default: `USD`)

### Examples

#### Request

#### Response

```
{
  "data": {
    "currency": "BTC",
    "rates": {
      "AED": "36.73",
      "AFN": "589.50",
      "ALL": "1258.82",
      "AMD": "4769.49",
      "ANG": "17.88",
      "AOA": "1102.76",
      "ARS": "90.37",
      "AUD": "12.93",
      "AWG": "17.93",
      "AZN": "10.48",
      "BAM": "17.38",
      ...
    }
  }
}

```