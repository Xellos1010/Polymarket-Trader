# deposits

Coinbase Prime supports receiving funds from external blockchain addresses as well as from other Coinbase wallets (e.g., Prime, Exchange, Coinbase.com), provided the asset is supported on Prime.

## Retrieving the default deposit address

To deposit funds into Coinbase Prime, a deposit address is required. Request one by calling [Get Wallet Deposit Instructions](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/get-wallet-deposit-instructions) with a relevant Wallet ID. This endpoint will always return the same address for a given wallet—it does **not** generate a new address for each request. The supported network will also be included in the response. Please review the [Wallets](https://developer.chrome.com/prime/concepts/wallets/wallets-overview) page before proceeding.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
WalletsService walletsService = PrimeServiceFactory.createWalletsService(client);
GetWalletDepositInstructionsRequest request = new GetWalletDepositInstructionsRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .walletId("WALLET_ID_HERE")
    .depositType(DepositType.CRYPTO)
    .build();
GetWalletDepositInstructionsResponse response = walletsService.getWalletDepositInstructions(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var walletsService = new WalletsService(client);
var request = new GetWalletDepositInstructionsRequest("PORTFOLIO_ID_HERE", "WALLET_ID_HERE")
{
    DepositType = DepositType.CRYPTO,
};
var response = walletsService.GetWalletDepositInstructions(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
walletsService := users.NewWalletsService(client)
request := &users.GetWalletDepositInstructionsRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    Id: "WALLET_ID_HERE",
    DepositType: "CRYPTO",
}
response, err := walletsService.GetWalletDepositInstructions(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.wallets import WalletsService, GetWalletDepositInstructionsRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
wallets_service = WalletsService(client)
request = GetWalletDepositInstructionsRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
    deposit_type="CRYPTO",
)
response = wallets_service.get_wallet_deposit_instructions(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl get-wallet-deposit-instructions --help

```

```
const walletsService = new WalletsService(client);
walletsService.getWalletDepositInstructions({
    portfolioId: 'PORTFOLIO_ID_HERE',
    walletId: 'WALLET_ID_HERE',
}).then(async (response) => {
    console.log('Deposit Instructions: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Creating unique deposit addresses

Prime also supports generating unique deposit addresses to differentiate between deposits into the same trading balance. This capability allows you to assign individual addresses for enhanced tracking and reconciliation purposes. This feature is particularly valuable for Crypto-as-a-Service (CaaS) clients and other use cases where distinguishing deposit sources is important. While the static deposit address approach is sufficient for most institutional trading scenarios, unique addresses provide additional tracking capabilities when needed. **Important**: Using unique deposit addresses for end user deposits typically requires enhanced due diligence during onboarding and KYC refresh processes. Please consult with your Coinbase Prime representative to ensure proper compliance procedures are in place. Use [Create Wallet Deposit Address](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet-deposit-address) to generate a new deposit address for a specific wallet and network. Unlike the static deposit instructions above, this endpoint generates a unique address for each request.

-   Python
    

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.wallets import WalletsService, CreateWalletDepositAddressRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
wallets_service = WalletsService(client)
request = CreateWalletDepositAddressRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
    network_id="ethereum-mainnet"
)
response = wallets_service.create_wallet_deposit_address(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

## Listing wallet addresses

You can retrieve all previously generated addresses for a wallet using [List Wallet Addresses](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/list-wallet-addresses). This is useful for auditing and tracking purposes.

-   Python
    

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.wallets import WalletsService, ListWalletAddressesRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
wallets_service = WalletsService(client)
request = ListWalletAddressesRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
    network_id="ethereum-mainnet"
)
response = wallets_service.list_wallet_addresses(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

## Tracking a deposit

Once a deposit is initiated (e.g., from an external wallet into a Prime wallet), its receipt can be tracked by polling the [List Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions) endpoint. Filter by the `DEPOSIT` transaction type to isolate deposit records. After a deposit is visible here, confirm its availability by checking current [balances](https://developer.chrome.com/prime/concepts/balances) (using either portfolio- or wallet-level balance endpoints). In general, digital asset deposits to Prime are credited once the relevant network confirmations have been met. For most assets, deposits become available for trading in a matter of seconds or minutes, but this timing may vary based on network conditions. For a complete implementation example of managing deposits and withdrawals with end-user subledgers, see the [Send & Receive reference application](https://github.com/coinbase-samples/prime-send-receive-go).

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
TransactionsService transactionsService = PrimeServiceFactory.createTransactionsService(client);
ListPortfolioTransactionsRequest request = new ListPortfolioTransactionsRequest.Builder()
.portfolioId("PORTFOLIO_ID_HERE")
.types("DEPOSIT")
.build();
ListPortfolioTransactionsResponse response = transactionsService.listPortfolioTransactions(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var transactionsService = new TransactionsService(client);
var request = new ListPortfolioTransactionsRequest("PORTFOLIO_ID_HERE", "DEPOSIT");
var response = transactionsService.ListPortfolioTransactions(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
transactionsService := transactions.NewTransactionsService(client)
request := &transactions.ListPortfolioTransactionsRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    Types: "DEPOSIT"
}
response, err := transactionsService.ListPortfolioTransactions(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.transactions import TransactionsService, ListPortfolioTransactionsRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
transactions_service = TransactionsService(client)
request = ListPortfolioTransactionsRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    types="DEPOSIT",
)
response = transactions_service.list_portfolio_transactions(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const transactionsService = new TransactionsService(client);
transactionsService.listPortfolioTransactions({
    portfolioId: 'PORTFOLIO_ID_HERE'
    types: 'DEPOSIT'
}).then(async (response) => {
    console.log('Transactions: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl list-portfolio-transactions --help

```

### Deposit Address Reconciliation

All deposit transactions include the specific deposit address that received the funds. Use this address to associate the deposit back to the addresses you generated and assigned to your users, payment invoices, or other tracking purposes. In the transaction response, the `transfer_to` object contains the exact deposit address as `address`. The `account_identifier` field will contain memo or destination tags for blockchain networks that require additional identifiers:

```
{
  "transfer_to": {
    "type": "WALLET",
    "value": "0b335245-4f8c-40f4-aed7-64301fdf741c",
    "address": "0x1234567890abcdef1234567890abcdef12345678",
    "account_identifier": ""
  }
}

```