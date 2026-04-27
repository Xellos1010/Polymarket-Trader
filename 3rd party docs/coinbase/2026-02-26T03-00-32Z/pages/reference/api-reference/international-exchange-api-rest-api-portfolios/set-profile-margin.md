# set profile margin

Set portfolio margin override

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
PortfoliosService portfoliosService = IntxServiceFactory.createPortfoliosService(client);
SetPortfolioMarginOverrideRequest request = new SetPortfolioMarginOverrideRequest.Builder().build();
SetPortfolioMarginOverrideResponse response = portfoliosService.setPortfolioMarginOverride(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var portfoliosService = new PortfoliosService(client);
var request = new SetPortfolioMarginOverrideRequest();
var response = portfoliosService.SetPortfolioMarginOverride(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
portfoliosSvc := portfolios.NewPortfoliosService(client)
request := &portfolios.SetMarginOverrideRequest{}
response, err := portfoliosSvc.SetMarginOverride(context.Background(), request)

```

For more information, please visit the [INTX Go SDK](https://github.com/coinbase-samples/intx-sdk-go).

```
client = IntxClient()
request = SetMarginOverrideRequest()
response = client.set_margin_override(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.updateMarginOverride({
    portfolioId: 'PORTFOLIO_ID_HERE',
    marginOverride: '0.1',
}).then(async (response) => {
    console.log('Margin Override Updated: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

```
intxctl set-margin-override --help

```

For more information, please visit the [INTX CLI](https://github.com/coinbase-samples/intx-cli).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Body

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`)

The margin override value for the portfolio. Specified as a decimal value representing notional requirement (e.g., 0.1 = 10% notional requirement, 0.25 = 25% notional requirement)

#### Response

Portfolio margin override set

The unique identifier of the portfolio the fill was executed under

The margin override value for the portfolio, indicating notional requirement to hold futures positions