# balances

Coinbase Prime offers real-time portfolio and wallet balance information through its API. Note that the endpoints described here do not retain historical balance data. If a use case requires a history of balances over time, these endpoints should periodically queried with the results stored in an external system for long-term recordkeeping.

## Calculating Portfolio Balances

Use [List Portfolio Balances](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/list-portfolio-balances) to see aggregated balances for all assets within a specific portfolio. This is useful for high-level monitoring and trading workflows, such as displaying a portfolio overview or verifying available funds before placing orders.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
BalancesService balancesService = PrimeServiceFactory.createBalancesService(client);
ListPortfolioBalancesRequest request = new ListPortfolioBalancesRequest.Builder("PORTFOLIO_ID_HERE").build();
ListPortfolioBalancesResponse response = balancesService.listPortfolioBalances(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var balancesService = new BalancesService(client);
var request = new ListPortfolioBalancesRequest("PORTFOLIO_ID_HERE");
var response = balancesService.ListPortfolioBalances(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
balancesService := balances.NewBalancesService(client)
request := &balances.ListPortfolioBalancesRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
}
response, err := balancesService.ListPortfolioBalances(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.balances import BalancesService, ListPortfolioBalancesRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
balances_service = BalancesService(client)
request = ListPortfolioBalancesRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
)
response = balances_service.list_portfolio_balances(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl list-portfolio-balances --help

```

```
const balancesService = new BalancesService(client);
balancesService.listPortfolioBalances({
    portfolioId: 'PORTFOLIO_ID_HERE',
    symbols: 'ETH'
}).then(async (response) => {
    console.log('Balances: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Calculating Wallet Balances

Individual wallet balances can be obtained with [Get Wallet Balance](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/get-wallet-balance). This endpoint returns the current balance for a single wallet, making it ideal for tracking balances at the **wallet level** — for instance, monitoring a specific vault wallet in Coinbase Custody or a trading wallet in Prime. Please review the [Wallets](https://developer.chrome.com/prime/concepts/wallets/wallets-overview) page before proceeding.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
BalancesService balancesService = PrimeServiceFactory.createBalancesService(client);
GetWalletBalanceRequest request = new GetWalletBalanceRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .walletId("WALLET_ID_HERE")
    .build();
GetWalletBalanceResponse response = balancesService.getWalletBalance(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var balancesService = new BalancesService(client);
var request = new GetWalletBalanceRequest("PORTFOLIO_ID_HERE", "WALLET_ID_HERE");
var response = balancesService.GetWalletBalance(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
balancesService := balances.NewBalancesService(client)
request := &balances.GetWalletBalance{
    PortfolioId: "PORTFOLIO_ID_HERE",
    WalletId: "WALLET_ID_HERE",
}
response, err := balancesService.GetWalletBalance(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.balances import BalancesService, GetWalletBalanceRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
balances_service = BalancesService(client)
request = GetWalletBalanceRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
)
response = balances_service.get_wallet_balance(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const balancesService = new BalancesService(client);
balancesService.getWalletBalance({
    portfolioId: 'PORTFOLIO_ID_HERE',
    walletId: 'WALLET_ID_HERE'
}).then(async (response) => {
    console.log('Balances: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl get-wallet-balance --help

```

Please note: All requests discussed above require proper authentication. For more information, visit [REST API Authentication](https://developer.chrome.com/prime/rest-api/authentication).