# get wallet by wallet id

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
WalletsService walletsService = PrimeServiceFactory.createWalletsService(client);
GetWalletByIdRequest request = new GetWalletByIdRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .walletId("WALLET_ID_HERE")
    .build();
GetWalletByIdResponse response = walletsService.getWalletById(request);

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var walletsService = new WalletsService(client);
var request = new GetWalletByIdRequest("PORTFOLIO_ID_HERE", "WALLET_ID_HERE");
var response = walletsService.GetWalletById(request);

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
walletsService := users.NewWalletsService(client)
request := &users.GetWalletRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    Id: "WALLET_ID_HERE",
}
response, err := walletsService.GetWallet(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = GetWalletRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
)
response = prime_client.get_wallet(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl get-wallet --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const walletsService = new WalletsService(client);
walletsService.getWallets({
    portfolioId: 'PORTFOLIO_ID_HERE',
    walletId: 'WALLET_ID_HERE'
}).then(async (response) => {
    console.log('Wallet: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Path Parameters

#### Response