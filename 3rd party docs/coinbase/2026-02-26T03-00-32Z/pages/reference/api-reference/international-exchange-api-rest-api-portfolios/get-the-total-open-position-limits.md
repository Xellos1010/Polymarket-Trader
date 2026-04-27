# get the total open position limits

Get the total open position limit for the portfolio

-   TS/JS
    

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.listOpenPositionLimits({
    portfolio: 'PORTFOLIO_ID_HERE',
}).then(async (response) => {
    console.log('Position Limits: ', response);
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

#### Response

Position limits information

total\_open\_position\_notional\_limit

The total notional limit across all instrument positions allowed to be opened. The value is not present if total\_open\_position\_notional\_limit\_enforced is false.

total\_open\_position\_notional\_limit\_enforced

Whether total notional limit across all instrument positions allowed to be opened is enforced.