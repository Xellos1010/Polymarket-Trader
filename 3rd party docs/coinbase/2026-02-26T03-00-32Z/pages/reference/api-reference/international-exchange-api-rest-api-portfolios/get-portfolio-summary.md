# get portfolio summary

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
PortfoliosService portfoliosService = IntxServiceFactory.createPortfoliosService(client);
GetPortfolioSummaryRequest request = new GetPortfolioSummaryRequest.Builder()
    .portfolio("portfolio_id")
    .build();
GetPortfolioSummaryResponse response = portfoliosService.getPortfolioSummary(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var portfoliosService = new PortfoliosService(client);
var request = new GetPortfolioSummaryRequest(
    Portfolio: "portfolio_id",
);
var response = portfoliosService.GetPortfolioSummary(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
portfoliosSvc := portfolios.NewPortfoliosService(client)
request := &portfolios.GetPortfolioSummaryRequest{
    Portfolio: "portfolio_id",
}
response, err := portfoliosSvc.GetPortfolioSummary(context.Background(), request)

```

For more information, please visit the [INTX Go SDK](https://github.com/coinbase-samples/intx-sdk-go).

```
client = IntxClient()
request = GetPortfolioSummaryRequest(
    portfolio="portfolio_id",
)
response = client.get_portfolio_summary(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.getPortfolioSummary({
    portfolio: 'PORTFOLIO_ID_HERE',
}).then(async (response) => {
    console.log('Portfolio Summary: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

```
intxctl get-portfolio-summary --help

```

For more information, please visit the [INTX CLI](https://github.com/coinbase-samples/intx-cli).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Path Parameters

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`)

#### Response

The total collateral value in USDC for the portfolio

The profit or loss of all open positions (resets to 0 after settlement)

The profit or loss percentage of all open positions (resets to 0 after settlement)

The position value in USDC of all positions

The notional value of all open orders and positions

Accrued fees that haven't been paid yet

Total borrow amount in USDC (nets the USDC balance, position PNL, held USDC, accrued interest and rolling debt)

Interest charged for borrowed USDC balances

Amount of settled transactions that haven't been paid

The net balance available in the portfolio (collateral + unrealized\_pnl - pending\_fees - accrued\_interest - rolling\_debt)

The amount of buying power available in the portfolio (balance - (open\_position\_notional \* portfolio\_initial\_margin))

The weighted average of all the position's initial margin utilization

The current margin level of the portfolio

portfolio\_maintenance\_margin

The maintenance margin of the portfolio

portfolio\_close\_out\_margin

The close out margin of the portfolio

Indicates whether the portfolio is in process of liquidation

portfolio\_initial\_margin\_notional

The notional initial margin of the portfolio

portfolio\_current\_margin\_notional

The notional current margin level of the portfolio

portfolio\_maintenance\_margin\_notional

The notional maintenance margin of the portfolio

portfolio\_close\_out\_margin\_notional

The notional close out margin of the portfolio

The margin override value for the portfolio, indicating notional requirement to hold futures positions

The effective margin requirement value for the portfolio `(max(portfolio_initial_margin, margin_override))`

loan\_collateral\_requirement

The notional value of margin relief provided by position offsets