# get portfolio by portfolio id

Get Portfolio by Portfolio ID

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
PortfoliosService portfoliosService = PrimeServiceFactory.createPortfoliosService(client);
GetPortfolioByIdRequest request = new GetPortfolioByIdRequest.Builder().portfolioId("PORTFOLIO_ID_HERE").build();
GetPortfolioByIdResponse response = portfoliosService.getPortfolioById(request);

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var portfoliosService = new PortfoliosService(client);
var request = new GetPortfolioByIdRequest("PORTFOLIO_ID_HERE");
var response = portfoliosService.GetPortfolioById(request);

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
portfoliosService := portfolios.NewPortfoliosService(client)
request := &portfolios.GetPortfolio{
    PortfolioId: "PORTFOLIO_ID_HERE",
}
response, err := portfoliosService.GetPortfolio(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = GetPortfolioRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
)
response = prime_client.get_portfolio(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl get-portfolio --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.getPortfolio({
    portfolioId: 'PORTFOLIO_ID_HERE'
}).then(async (response) => {
    console.log('Portfolio: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Path Parameters

#### Response