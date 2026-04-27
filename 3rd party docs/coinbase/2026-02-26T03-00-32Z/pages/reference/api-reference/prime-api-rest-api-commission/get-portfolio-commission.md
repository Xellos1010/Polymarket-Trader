# get portfolio commission

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
CommissionService commissionService = PrimeServiceFactory.createCommissionService(client);
GetPortfolioCommissionRequest request = new GetPortfolioCommissionRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .build();
GetPortfolioCommissionResponse response = commissionService.getPortfolioCommission(request);

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var commissionService = new CommissionService(client);
var request = new GetPortfolioCommissionRequest("PORTFOLIO_ID_HERE");
var response = commissionService.GetPortfolioCommission(request);

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
commissionService = commission.NewCommissionService(client)
request := &commission.GetPortfolioCommissionRequest{
    PortfolioId: "portfolio-id",
}
response, err := commissionService.GetPortfolioCommission(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = GetPortfolioCommissionRequest(
    portfolio_id="portfolio-id",
)
response = prime_client.get_portfolio_commission(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl get-commission --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const commissionService = new CommissionService(client);
commissionService.getPortfolioCommission({
    portfolioId: 'PORTFOLIO_ID_HERE'
}).then(async (response) => {
    console.log('Commission: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Path Parameters

#### Query Parameters

Specific trading pair to check commission (e.g BTC-USD)

#### Response