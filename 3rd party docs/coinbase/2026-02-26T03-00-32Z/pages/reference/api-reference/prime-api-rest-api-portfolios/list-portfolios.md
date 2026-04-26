# list portfolios

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
PortfoliosService portfoliosService = PrimeServiceFactory.createPortfoliosService(client);
ListPortfoliosResponse response = portfoliosService.listPortfolios();

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var portfoliosService = new PortfoliosService(client);
var response = portfoliosService.ListPortfolios();

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
portfoliosService := portfolios.NewPortfoliosService(client)
request := &portfolios.ListPortfolios{}
response, err := portfoliosService.ListPortfolios(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = ListPortfoliosRequest()
response = prime_client.list_portfolios(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl list-portfolios --entity-id ENTITY_ID_HERE

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.listPortfolios().then(async (response) => {
    console.log('Portfolios: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Response