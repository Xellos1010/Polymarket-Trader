# get fund transfer limit

Get fund transfer limit between portfolios

-   TS/JS
    

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.getFundTransferLimit({
    portfolio: 'PORTFOLIO_ID_HERE',
    asset: 'ETH',
}).then(async (response) => {
    console.log('Fund Transfer Limit: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Path Parameters

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`)

Identifies the asset by name (e.g., `BTC`), UUID (e.g., `291efb0f-2396-4d41-ad03-db3b2311cb2c`), or asset ID (e.g., `1482439423963469`)

#### Response

max\_portfolio\_transfer\_amount

max portfolio transfer amount