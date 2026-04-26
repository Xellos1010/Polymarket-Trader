# cancel pending futures sweep

##### REST API

-   [
    
    Overview
    
    
    
    ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/introduction)

-   -   [DEL
        
        Cancel Pending US Derivatives Sweep
        
        
        
        ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/cancel-pending-futures-sweep)
    -   [GET
        
        Get Current Margin Window
        
        
        
        ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/get-current-margin-window)
    -   [GET
        
        Get US Derivatives Balance Summary
        
        
        
        ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/get-futures-balance-summary)
    -   [GET
        
        Get US Derivatives Position
        
        
        
        ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/get-futures-position)
    -   [GET
        
        Get Intraday Margin Setting
        
        
        
        ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/get-intraday-margin-setting)
    -   [GET
        
        List US Derivatives Positions
        
        
        
        ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/list-futures-positions)
    -   [GET
        
        List US Derivatives Sweeps
        
        
        
        ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/list-futures-sweeps)
    -   [POST
        
        Schedule US Derivatives Sweep
        
        
        
        ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/schedule-futures-sweep)
    -   [POST
        
        Set Intraday Margin Setting
        
        
        
        ](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/set-intraday-margin-settings)

Cancel Pending Futures Sweep

```
curl --request DELETE \
  --url https://api.coinbase.com/api/v3/brokerage/cfm/sweeps \
  --header 'Authorization: Bearer <token>'
```

```
{
  "success": true
}
```

US Derivatives

Cancel the pending sweep of funds from FCM wallet to USD Spot wallet

DELETE

/

api

/

v3

/

brokerage

/

cfm

/

sweeps

Cancel Pending Futures Sweep

```
curl --request DELETE \
  --url https://api.coinbase.com/api/v3/brokerage/cfm/sweeps \
  --header 'Authorization: Bearer <token>'
```

```
{
  "success": true
}
```

#### Authorizations

[​](#authorization-authorization)

Authorization

string

header

required

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Response

A successful response.

[​](#response-success)

success

boolean

Was this page helpful?

[

Get Transaction Summary

Previous

](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/fees/get-transaction-summary)[

Get Current Margin Window

Next

](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/get-current-margin-window)

⌘I