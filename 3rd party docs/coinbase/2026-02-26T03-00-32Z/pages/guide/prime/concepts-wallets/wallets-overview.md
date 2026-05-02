# wallets overview

The concept of a **wallet** is central to Prime’s architecture. Each wallet has a unique **Wallet ID**, and there are three main types:

-   **Trading Balance**: A wallet that is part of an omnibus account, directly available for trading. Each asset within a portfolio has exactly one Trading Balance Wallet ID.
-   **Custodial Vault Wallet**: A wallet maintained within Coinbase Custody, where assets are held offline and segregated. Multiple vault wallets may be created for each asset by calling [Create Wallet](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet). Each vault wallet has its own Wallet ID.
-   **Onchain Wallet**: To learn more about this wallet type, visit [Onchain Wallet](https://developer.chrome.com/prime/concepts/wallets/onchain-wallet)

The Wallet ID is required in many Prime API requests, such as those for querying balances, initiating transactions, and retrieving deposit instructions.

## Creating a Wallet

**Trading Balance Wallets** are automatically created under one of the following conditions:

-   When a user first navigates to an asset in the Prime UI
-   When a user places a trade for that asset
-   After calling [Create Wallet](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet) with the `TRADING` wallet type

To create a **Vault Wallet**, the [Create Wallet](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet) endpoint must be called with the `VAULT` type. Below are examples of this process. After a wallet creation request for a vault wallet is submitted, the response will include an **Activity ID**. Since most actions within Prime require consensus approval, a review step in the UI must be completed. Once the wallet creation is approved, the new wallet’s details, including its Wallet ID, can be looked up.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
WalletsService walletsService = PrimeServiceFactory.createWalletsService(client);
CreateWalletRequest request = new CreateWalletRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .type(WalletType.VAULT)
    .name("PRIME_API_EXAMPLE")
    .symbol("ETH")
    .build();
CreateWalletResponse response = walletsService.createWallet(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var walletsService = new WalletsService(client);
var request = new CreateWalletRequest("PORTFOLIO_ID_HERE")
{
    Type = WalletType.VAULT,
    Name = "PRIME_API_EXAMPLE",
    Symbol = "ETH",
}
var response = walletsService.CreateWallet(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
walletsService := users.NewWalletsService(client)
request := &users.CreateWalletRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    Type: "VAULT",
    Name: "PRIME_API_EXAMPLE",
    Symbol: "ETH",
}
response, err := walletsService.CreateWallet(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.wallets import WalletsService, CreateWalletRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
wallets_service = WalletsService(client)
request = CreateWalletRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    name="PRIME_API_EXAMPLE",
    symbol="ETH",
    wallet_type="VAULT",
)
response = wallets_service.create_wallet(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl create-wallet --help

```

```
const walletsService = new WalletsService(client);
walletsService.getWalletDepositInstructions({
    portfolioId: 'PORTFOLIO_ID_HERE',
    type: WalletType.VAULT,
    name: "PRIME_API_EXAMPLE",
    symbol: "ETH",
}).then(async (response) => {
    console.log('Wallet: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Finding a Wallet ID

Use the [List Portfolio Wallets](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/list-portfolio-wallets) endpoint to retrieve all wallets for a portfolio, optionally filtering by wallet type or other parameters. If preferred, wallets may be located by name using a utility script, such as the [Get Wallet by Name](https://github.com/coinbase-samples/prime-scripts-py/blob/main/REST/prime_get_wallet_by_name.py) example from the Coinbase Samples [scripts repository](https://github.com/coinbase-samples/prime-scripts-py). Below is an example of how to list all vault wallets for a portfolio.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
WalletsService walletsService = PrimeServiceFactory.createWalletsService(client);
ListWalletsRequest request = new ListWalletsRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .type(WalletType.VAULT)
    .build();
ListWalletsResponse response = walletsService.listWallets(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var walletsService = new WalletsService(client);
var request = new ListWalletsRequest("PORTFOLIO_ID_HERE")
{
    Type = WalletType.VAULT,
}
var response = walletsService.ListWallets(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
walletsService := users.NewWalletsService(client)
request := &users.ListWalletsRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    Type: "VAULT",
}
response, err := walletsService.ListWallets(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.wallets import WalletsService, ListWalletsRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
wallets_service = WalletsService(client)
request = ListWalletsRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    type="VAULT",
)
response = wallets_service.list_wallets(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const walletsService = new WalletsService(client);
walletsService.listWallets({
    portfolioId: 'PORTFOLIO_ID_HERE',
    type: WalletType.VAULT,
}).then(async (response) => {
    console.log('Wallets: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl list-wallets --help

```

Please note: All requests discussed above require proper authentication. For more information, visit [REST API Authentication](https://developer.chrome.com/prime/rest-api/authentication).