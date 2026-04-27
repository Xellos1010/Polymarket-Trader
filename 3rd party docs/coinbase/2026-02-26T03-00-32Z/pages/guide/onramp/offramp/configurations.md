# configurations

## Offramp Config

The Offramp Config API returns the list of countries that support offramp transactions, and the payment methods available in each country. Clients can call this API periodically and cache the response so that they know which users to present the Coinbase Offramp sell transaction option to.

### Method

### URL

```
https://api.developer.coinbase.com/onramp/v1/sell/config

```

### Request Parameters

The Offramp Config API has no request parameters.

### Response Fields

The Offramp Config API returns a JSON response including the following fields.

Name

Description

`countries`

A list of supported countries, represented by their [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) two digit country code. Each country contains a list of payment method types available in that country; for the US it also contains a list of supported states.

### Example Request/Response

-   Request (cURL)
    
-   Response 200 (JSON)
    

```
cdpcurl -k /tmp/cdp_api_key.json 'https://api.developer.coinbase.com/onramp/v1/sell/config'

```

```
{
  "data": {
    "countries": [
      {
        "id": "US",
        "subdivisions": ["CA","NY","WA"],
        "payment_methods": [
          {
            "id": "CRYPTO_WALLET",
          },
          {
            "id": "FIAT_WALLET",
          },
          {
            "id": "ACH_BANK_ACCOUNT",
          },
        ],
      },
      {
        "id": "CA",
        "subdivisions": []
        "payment_methods": [
          {
            "id": "CARD",
          },
        ],
      },
    ],
  }
}

```

## Offramp Options

The Offramp Options API provides clients with a way to discover the available options for selling Crypto with Coinbase Offramp. It returns the supported fiat currencies and available crypto assets that can be passed into the Sell Quote API.

### Method

### URL

```
https://api.developer.coinbase.com/onramp/v1/sell/options

```

### Request Parameters

Name

Req

Description

`country`

Y

[ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) two-digit country code string representing the purchasing user’s country of residence, e.g., `US`.

`subdivision`

N

[ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) two-digit country subdivision code representing the purchasing user’s subdivision of residence within their country, e.g. `NY`. Required if the `country=“US”` because certain states (e.g., `NY`) have state specific asset restrictions.

### Response Fields

The Offramp Options API returns a JSON response including the following fields.

Name

Description

`cashout_currencies`

List of supported fiat currencies that can exchange crypto to on Coinbase Onramp in the given location. Each currency contains a list of available payment methods, with min and max transaction limits for that currency.

`sell_currencies`

List of available crypto assets that can be sold on Onramp in the given location.

### Example Request/Response

-   Request (cURL)
    
-   Response 200 (JSON)
    

```
cdpcurl -k /tmp/cdp_api_key.json 'https://api.developer.coinbase.com/onramp/v1/sell/options?country=US&subdivision=NY'

```

```
{
  "data": {
    "cashout_currencies": [
      {
        "id": "USD",
        "payment_method_limits": [
          {
            "id": "card",
            "min": "10.00",
            "max": "7500.00",
          },
          {
            "id": "ach_bank_account",
            "min": "10.00",
            "max": "25000.00",
          },
        ],
      },
    ],
    "sell_currencies": [
      {
        "id": "2b92315d-eab7-5bef-84fa-089a131333f5",
        "name": "USD Coin",
        "symbol": "USDC",
        "networks": [
          {
            "name": "ethereum-mainnet",
            "display_name": "Ethereum",
            "chain_id": "1",
            "contract_address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
          },
          {
            "name": "polygon-mainnet",
            "display_name": "Polygon",
            "chain_id": "137",
            "contract_address": "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
          },
        ]
      }
    ],
  }
}

```