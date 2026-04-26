# list financing eligible assets

##### REST API

-   [
    
    Overview
    
    
    
    ](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)
-   [
    
    Prime Overview
    
    
    
    ](https://developer.chrome.com/prime/concepts/overview)
-   [
    
    Requests
    
    
    
    ](https://developer.chrome.com/prime/rest-api/requests)
-   [
    
    Rate Limits
    
    
    
    ](https://developer.chrome.com/prime/rest-api/rate-limits)
-   [
    
    Authentication
    
    
    
    ](https://developer.chrome.com/prime/rest-api/authentication)
-   [
    
    Pagination
    
    
    
    ](https://developer.chrome.com/prime/rest-api/pagination)
-   [
    
    CLI Setup
    
    
    
    ](https://developer.chrome.com/prime/rest-api/cli-setup)
-   [
    
    SDKs
    
    
    
    ](https://developer.chrome.com/prime/rest-api/sdks)
-   [
    
    Types
    
    
    
    ](https://developer.chrome.com/prime/rest-api/types)

-   -   [GET
        
        Get Cross Margin Overview
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-cross-margin-overview)
    -   [GET
        
        Get Entity Locate Availabilities
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-entity-locate-availabilities)
    -   [GET
        
        Get Margin Information
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-margin-information)
    -   [GET
        
        Get Portfolio Buying Power
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-buying-power)
    -   [GET
        
        Get Portfolio Credit Information
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-credit-information)
    -   [GET
        
        Get Portfolio Withdrawal Power
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-withdrawal-power)
    -   [GET
        
        Get Trade Finance Tiered Pricing Fees
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-trade-finance-tiered-pricing-fees)
    -   [GET
        
        List Existing Locates
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-existing-locates)
    -   [GET
        
        List Financing Eligible Assets
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-financing-eligible-assets)
    -   [GET
        
        List Interest Accruals
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-interest-accruals)
    -   [GET
        
        List Interest Accruals For Portfolio
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-interest-accruals-for-portfolio)
    -   [GET
        
        List Margin Call Summaries
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-margin-call-summaries)
    -   [GET
        
        List Margin Conversions
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-margin-conversions)
    -   [GET
        
        List Trade Finance Obligations
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-trade-finance-obligations)
    -   [POST
        
        Create New Locates
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/create-new-locates)

##### FIX API

-   [
    
    Connectivity
    
    
    
    ](https://developer.chrome.com/prime/fix-api/connectivity)

##### Websocket Feed

-   [
    
    Overview
    
    
    
    ](https://developer.chrome.com/prime/websocket-feed/overview)
-   [
    
    Channels
    
    
    
    ](https://developer.chrome.com/prime/websocket-feed/channels)

List Financing Eligible Assets

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/financing/eligible-assets
```

```
{
  "assets": [
    {
      "symbol": "BTC",
      "asset_adjustment": "0.85",
      "liability_adjustment": "1.15"
    }
  ]
}
```

Financing

Get all assets eligible for Trade Finance with their adjustment factors.

GET

/

v1

/

financing

/

eligible-assets

List Financing Eligible Assets

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/financing/eligible-assets
```

```
{
  "assets": [
    {
      "symbol": "BTC",
      "asset_adjustment": "0.85",
      "liability_adjustment": "1.15"
    }
  ]
}
```

### 

[​](#supported-products)

Supported Products

-   Trade Finance

#### Response

200 - application/json

A successful response.

[​](#response-assets)

assets

TFAsset represents an asset eligible for Trade Finance with adjustment factors · object\[\]

List of assets eligible for Trade Finance

Show child attributes

Was this page helpful?

[

List Existing Locates

Previous

](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-existing-locates)[

List Interest Accruals

Next

](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-interest-accruals)

⌘I