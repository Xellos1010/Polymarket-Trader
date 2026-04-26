# set fcm settings

##### REST API

-   [](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)
-   [](https://developer.chrome.com/prime/concepts/overview)
-   [](https://developer.chrome.com/prime/rest-api/requests)
-   [](https://developer.chrome.com/prime/rest-api/rate-limits)
-   [](https://developer.chrome.com/prime/rest-api/authentication)
-   [](https://developer.chrome.com/prime/rest-api/pagination)
-   [](https://developer.chrome.com/prime/rest-api/cli-setup)
-   [](https://developer.chrome.com/prime/rest-api/sdks)
-   [](https://developer.chrome.com/prime/rest-api/types)

-   -   [GET](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-entity-fcm-balance)
    -   [GET](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-entity-positions)
    -   [GET](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-fcm-equity)
    -   [GET](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-fcm-margin-call-details)
    -   [GET](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-fcm-risk-limits)
    -   [GET](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-fcm-settings)
    -   [GET
        
        List Entity Futures Sweeps
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/list-entity-futures-sweeps)
    -   [POST
        
        Schedule Entity Futures Sweep
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/schedule-entity-futures-sweep)
    -   [POST](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/set-auto-sweep)
    -   [POST](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/set-fcm-settings)
    -   [DEL
        
        Cancel Entity Futures Sweep
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/cancel-entity-futures-sweep)

##### FIX API

-   [](https://developer.chrome.com/prime/fix-api/connectivity)

##### Websocket Feed

-   [](https://developer.chrome.com/prime/websocket-feed/overview)
-   [](https://developer.chrome.com/prime/websocket-feed/channels)

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/settings \
  --header 'Content-Type: application/json' \
  --data '
{
  "target_derivatives_excess": "1000.00"
}
'
```

Update settings related to FCM.

POST

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

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/settings \
  --header 'Content-Type: application/json' \
  --data '
{
  "target_derivatives_excess": "1000.00"
}
'
```

#### Path Parameters

#### Body

target\_derivatives\_excess

Target CFM Excess amount to set. Only non-negative number is allowed

#### Response