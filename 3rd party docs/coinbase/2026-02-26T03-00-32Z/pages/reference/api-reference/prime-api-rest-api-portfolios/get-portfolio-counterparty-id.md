# get portfolio counterparty id

-   [Get help](https://discord.com/invite/cdp)
-   [Dev portal](https://portal.cdp.coinbase.com/)
-   [](https://portal.cdp.coinbase.com/)

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

-   -   [GET
        
        Get Portfolio by Portfolio ID
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/portfolios/get-portfolio-by-portfolio-id)
    -   [GET
        
        Get Portfolio Counterparty ID
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/portfolios/get-portfolio-counterparty-id)
    -   [GET](https://developer.chrome.com/api-reference/prime-api/rest-api/portfolios/list-portfolios)

##### FIX API

-   [](https://developer.chrome.com/prime/fix-api/connectivity)

##### Websocket Feed

-   [](https://developer.chrome.com/prime/websocket-feed/overview)
-   [](https://developer.chrome.com/prime/websocket-feed/channels)

Get Portfolio Counterparty ID

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/counterparty
```

```
{
  "counterparty": {
    "counterparty_id": "CB12345678"
  }
}
```

Retrieve the counterparty ID for a given portfolio.

GET

/

v1

/

portfolios

/

{portfolio\_id}

/

counterparty

Get Portfolio Counterparty ID

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/counterparty
```

```
{
  "counterparty": {
    "counterparty_id": "CB12345678"
  }
}
```

#### Path Parameters

#### Response