# preview unstake

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

-   -   [GET](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/get-staking-status)
    -   [GET](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/get-unstaking-status)
    -   [POST
        
        Claim Wallet Staking Rewards
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/claim-wallet-staking-rewards-alpha)
    -   [POST
        
        Query Transaction Validators
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/list-transaction-validators)
    -   [POST](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/preview-unstake)
    -   [POST
        
        Request to stake currency in a portfolio
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/request-to-stake-currency-in-a-portfolio)
    -   [POST
        
        Request to stake or delegate a wallet
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/request-to-stake-or-delegate-a-wallet)
    -   [POST
        
        Request to unstake a wallet
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/request-to-unstake-a-wallet)
    -   [POST
        
        Request to unstake currency across a portfolio
        
        
        
        ](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/request-to-unstake-currency-across-a-portfolio)

##### FIX API

-   [](https://developer.chrome.com/prime/fix-api/connectivity)

##### Websocket Feed

-   [](https://developer.chrome.com/prime/websocket-feed/overview)
-   [](https://developer.chrome.com/prime/websocket-feed/channels)

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/wallets/{wallet_id}/staking/unstake/preview \
  --header 'Content-Type: application/json' \
  --data '
{
  "amount": "<string>"
}
'
```

```
{
  "estimated_amount": "<string>"
}
```

Previews an unstaking request with the given amount and returns the estimated amount that would be unstaked. This feature currently only supports ETH.

POST

/

v1

/

portfolios

/

{portfolio\_id}

/

wallets

/

{wallet\_id}

/

staking

/

unstake

/

preview

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/wallets/{wallet_id}/staking/unstake/preview \
  --header 'Content-Type: application/json' \
  --data '
{
  "amount": "<string>"
}
'
```

```
{
  "estimated_amount": "<string>"
}
```

#### Path Parameters

#### Body

PreviewUnstakeRequest represents a request to preview an unstaking operation.

Amount to preview unstaking

#### Response

PreviewUnstakeResponse contains the response data from previewing an unstaking operation.

Estimated amount that would be unstaked