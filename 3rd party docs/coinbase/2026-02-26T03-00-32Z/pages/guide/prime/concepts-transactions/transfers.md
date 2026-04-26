# transfers

Coinbase Prime supports **onchain transfers** of digital assets between Prime **portfolios** under the same entity. This functionality is useful for moving funds from a trading balance to a vault wallet or consolidating assets across multiple portfolios. For those unfamiliar with portfolios or entities, review the [Account Structure](https://developer.chrome.com/prime/concepts/account-structure) page first.

## Creating a transfer

Instead of using a public address, the [Create Transfer](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-transfer) endpoint operates with **Wallet IDs**. For a refresher on Wallet IDs, consult the [Wallets](https://developer.chrome.com/prime/concepts/wallets/wallets-overview) page. When creating a transfer, specify the **source wallet ID** (and its associated portfolio) as well as the **destination wallet ID**. To perform a transfer between two portfolios, an entity or organization-level API key is necessary to keep all wallets in scope. Transfers are subject to Prime’s consensus approval process, which can be configured in the Prime UI. Below is an example showing how to create a transfer.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
TransactionsService transactionsService = PrimeServiceFactory.createTransactionsService(client);
CreateTransferRequest request = new CreateTransferRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .walletId("WALLET_ID_HERE")
    .amount("0.001")
    .destination("DESTINATION_WALLET_UUID")
    .idempotencyKey(UUID.randomUUID().toString())
    .currencySymbol("ETH")
    .build();
CreateTransferResponse response = transactionsService.createTransfer(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var transactionsService = new TransactionsService(client);
var request = new CreateTransferRequest("PORTFOLIO_ID_HERE", "WALLET_ID_HERE")
{
    Amount = "0.001",
    Destination = "DESTINATION_WALLET_UUID",
    IdempotencyKey = Guid.NewGuid().ToString(),
    CurrencySymbol = "ETH",
};
var response = transactionsService.CreateTransfer(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
transactionsService := transactions.NewTransactionsService(client)
request := &transactions.CreateWalletTransferRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    WalletId: "WALLET_ID_HERE",
    Amount: "0.001",
    Destination: "DESTINATION_WALLET_UUID",
    IdempotencyKey: uuid.New().String(),
    CurrencySymbol: "ETH",
}
response, err := transactionsService.CreateWalletTransfer(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.transactions import TransactionsService, CreateTransferRequest
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
transactions_service = TransactionsService(client)
request = CreateTransferRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
    amount = '0.001',
    destination = 'DESTINATION_WALLET_UUID',
    idempotency_key = str(uuid.uuid4()),
    currency_symbol = 'ETH',
)
response = transactions_service.create_transfer(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const transactionsService = new TransactionsService(client);
transactionsService.createTransfer({
    portfolioId: 'PORTFOLIO_ID_HERE',
    walletId: 'WALLET_ID_HERE',
    amount: "0.001",
    destination: "DESTINATION_WALLET_UUID",
    idempotencyKey: uuidv4(),
    currencySymbol: "ETH",
}).then(async (response) => {
    console.log('Transfer: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl create-transfer --help

```

## Tracking Transfer Status

Use the **Transaction ID** returned by the Create Transfer endpoint to track the transaction’s status:

-   **[Get Transaction by ID](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/get-transaction-by-transaction-id)**: Retrieves detailed information for a single transaction.
-   **[List Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions)**: Can filter by `WITHDRAWAL` to list all withdrawal transactions.

Please note: All requests discussed above require proper authentication. For more information, visit [REST API Authentication](https://developer.chrome.com/prime/rest-api/authentication).