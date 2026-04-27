# acquire and repay loan

-   TS/JS
    

```
const positionOffsetsService = new PositionOffsetsService(client);
positionOffsetsService.acquireOrRepayLoan({
    portfolio: 'PORTFOLIO_ID_HERE',
    asset: 'ETH',
    action: LoanUpdateAction.ACQUIRE,
    amount: '1',
}).then(async (response) => {
    console.log('Active Loan: ', response);
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

#### Body

The loan update action to take

Available options

:

`ACQUIRE`,

`REPAY`

The amount of the loan to acquire or repay in quantity

#### Response

The unique identifier of the portfolio

The unique identifier of the asset

The change in the loan amount

The total quantity of the asset borrowed by the portfolio

The UUID of the asset

Example:

`"d92669ba-8a04-46d8-9b28-a2bbaeee3b9a"`

The UUID of the portfolio

Example:

`"018ab3b1-d38a-750e-8a1d-8b7815ea8bfb"`