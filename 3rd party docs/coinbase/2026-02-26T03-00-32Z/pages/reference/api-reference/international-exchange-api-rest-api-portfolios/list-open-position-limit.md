# list open position limit

List the open position limits for all instruments

-   TS/JS
    

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.getInstrumentPositionLimit({
    portfolio: 'PORTFOLIO_ID_HERE',
    instrument: 'ETH',
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

Name of the instrument the position is in

The unique identifier of the instrument the position is in

The UUID of the instrument the position is in

Example:

`"8ca6c040-48df-426b-bb4e-74413909da26"`

open\_position\_notional\_limit

The notional limit of the instrument position allowed to be opened