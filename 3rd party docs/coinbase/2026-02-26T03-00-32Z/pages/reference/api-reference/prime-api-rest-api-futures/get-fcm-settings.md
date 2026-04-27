# get fcm settings

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
        
        Get Entity FCM Balance
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-entity-fcm-balance)
    -   [GET
        
        Get Entity Positions
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-entity-positions)
    -   [GET
        
        Get FCM Equity
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-fcm-equity)
    -   [GET
        
        Get Margin Call Details
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-fcm-margin-call-details)
    -   [GET
        
        Get Risk Limits
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-fcm-risk-limits)
    -   [GET
        
        Get FCM Settings
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-fcm-settings)
    -   [GET
        
        List Entity Futures Sweeps
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/list-entity-futures-sweeps)
    -   [POST
        
        Schedule Entity Futures Sweep
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/schedule-entity-futures-sweep)
    -   [POST
        
        Set Auto Sweep
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/set-auto-sweep)
    -   [POST
        
        Set FCM Settings
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/set-fcm-settings)
    -   [DEL
        
        Cancel Entity Futures Sweep
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/cancel-entity-futures-sweep)

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

Get FCM Settings

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/settings
```

```
{
  "target_derivatives_excess": "1000.00"
}
```

Futures

Get settings related to FCM.

GET

/

v1

/

entities

/

{entity\_id}

/

futures

/

settings

Get FCM Settings

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/settings
```

```
{
  "target_derivatives_excess": "1000.00"
}
```

#### Path Parameters

[​](#parameter-entity-id)

entity\_id

string

required

Entity ID

#### Response

200 - application/json

A successful response.

[​](#response-target-derivatives-excess)

target\_derivatives\_excess

string

Target derivatives excess in the FCM

Example:

`"1000.00"`

Was this page helpful?

[

Get Risk Limits

Previous

](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-fcm-risk-limits)[

List Entity Futures Sweeps

Next

](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/list-entity-futures-sweeps)

⌘I