# create transfer

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

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

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

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

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

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

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = CreateTransferRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
    amount = '0.001',
    destination = 'DESTINATION_WALLET_UUID',
    idempotency_key = str(uuid.uuid4()),
    currency_symbol = 'ETH',
)
response = prime_client.create_transfer(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl create-transfer --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

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

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Path Parameters

The wallet ID that the transfer will originate from

#### Body

The amount in whole units to send

The UUID of the destination wallet

The idempotency key associated with this transfer

The currency symbol to transfer

#### Response

The activity ID for the transfer

A URL to the activity associated with this transfer for approval

The currency symbol of the transfer

The amount of the transfer

The network fee associated with the transfer

The destination address of the transfer

The destination type of the transfer

The source address used for the transfer

The source type used for the transfer

The id of the just created transaction