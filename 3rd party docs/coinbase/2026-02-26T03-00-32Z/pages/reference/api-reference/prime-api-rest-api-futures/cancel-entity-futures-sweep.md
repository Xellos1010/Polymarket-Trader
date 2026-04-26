# cancel entity futures sweep

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

Cancel Entity Futures Sweep

```
curl --request DELETE \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/sweeps
```

```
{
  "success": true,
  "request_id": "00000000-0000-0000-0000-000000000000"
}
```

Cancel the pending sweep for a given entity. A user will only be able to have one pending sweep at a time. If the sweep is not found, a 404 will be returned.

DELETE

/

v1

/

entities

/

{entity\_id}

/

futures

/

sweeps

Cancel Entity Futures Sweep

```
curl --request DELETE \
  --url https://api.prime.coinbase.com/v1/entities/{entity_id}/futures/sweeps
```

```
{
  "success": true,
  "request_id": "00000000-0000-0000-0000-000000000000"
}
```

#### Path Parameters

#### Response

Request ID

Example:

`"00000000-0000-0000-0000-000000000000"`