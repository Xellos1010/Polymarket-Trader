# delete existing travel rule

##### REST API

-   [](https://developer.chrome.com/api-reference/exchange-api/rest-api/introduction)
-   [](https://developer.chrome.com/exchange/rest-api/requests)
-   [](https://developer.chrome.com/exchange/rest-api/authentication)
-   [](https://developer.chrome.com/exchange/rest-api/rate-limits)
-   [](https://developer.chrome.com/exchange/rest-api/pagination)
-   [](https://developer.chrome.com/exchange/rest-api/sdks)
-   [](https://developer.chrome.com/exchange/rest-api/types)

-   -   [POST](https://developer.chrome.com/api-reference/exchange-api/rest-api/travel-rules/create-travel-rule)
    -   [DEL
        
        Delete existing travel rule entry
        
        
        
        ](https://developer.chrome.com/api-reference/exchange-api/rest-api/travel-rules/delete-existing-travel-rule)
    -   [GET
        
        Get all travel rule information
        
        
        
        ](https://developer.chrome.com/api-reference/exchange-api/rest-api/travel-rules/get-all-travel-rule)

##### FIX API

-   [](https://developer.chrome.com/exchange/fix-api/connectivity)
-   [](https://developer.chrome.com/exchange/fix-api/best-practices)
-   [](https://developer.chrome.com/exchange/fix-api/rate-limits)
-   [](https://developer.chrome.com/exchange/fix-api/drop-copy)

-   [](https://developer.chrome.com/exchange/fix-api/market-data)
-   [](https://developer.chrome.com/exchange/fix-api/dictionary-downloads)

##### Websocket Feed

-   [](https://developer.chrome.com/exchange/websocket-feed/overview)
-   [](https://developer.chrome.com/exchange/websocket-feed/best-practices)
-   [](https://developer.chrome.com/exchange/websocket-feed/authentication)
-   [](https://developer.chrome.com/exchange/websocket-feed/channels)
-   [](https://developer.chrome.com/exchange/websocket-feed/rate-limits)
-   [](https://developer.chrome.com/exchange/websocket-feed/errors)

Delete existing travel rule entry

```
curl --request DELETE \
  --url https://api.exchange.coinbase.com/travel-rules/{id} \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

Delete existing travel rule entry

Delete existing travel rule entry

```
curl --request DELETE \
  --url https://api.exchange.coinbase.com/travel-rules/{id} \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

#### Authorizations

#### Path Parameters

#### Response

The response is of type `object`.