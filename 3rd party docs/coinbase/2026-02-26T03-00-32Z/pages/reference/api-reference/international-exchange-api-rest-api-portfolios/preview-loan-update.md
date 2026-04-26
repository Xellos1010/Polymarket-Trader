# preview loan update

-   TS/JS
    

```
const positionOffsetsService = new PositionOffsetsService(client);
positionOffsetsService.previewLoanUpdate({
    portfolio: 'PORTFOLIO_ID_HERE',
    asset: 'ETH',
    action: LoanUpdateAction.ACQUIRE,
    amount: '1',
}).then(async (response) => {
    console.log('Preview Loan: ', response);
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

Loan update preview result

initial\_margin\_contribution

The notional amount this loan contributes to the portfolio initial margin notional value

The delta of notional amount this loan contributes to the portfolio initial margin notional value

The weighted average of all initial margin utilization

portfolio\_initial\_margin\_notional

The notional initial margin of the portfolio

loan\_collateral\_requirement

The notional collateral requirement to hold the loan

loan\_collateral\_requirement\_delta

The delta of notional collateral requirement to hold the loan

The total quantity of the asset borrowed by the portfolio

The delta of the total quantity of the asset borrowed by the portfolio

The maximum remaining amount of this asset that can be borrowed at this time

The reason for the rejection of the loan preview request

Indicates whether the requested loan preview could be executed