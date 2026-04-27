# create conversion

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
TransactionsService transactionsService = PrimeServiceFactory.createTransactionsService(client);
CreateConversionRequest request = new CreateConversionRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .walletId("WALLET_ID_HERE")
    .amount("1")
    .destination("DESTINATION_WALLET_UUID")
    .idempotencyKey(UUID.randomUUID().toString())
    .sourceSymbol("USD")
    .destinationSymbol("USDC")
    .build();
CreateConversionResponse response = transactionsService.createConversion(request);

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var transactionsService = new TransactionsService(client);
var request = new CreateConversionRequest("PORTFOLIO_ID_HERE", "WALLET_ID_HERE")
{
    Amount = "1",
    Destination = "DESTINATION_WALLET_UUID",
    IdempotencyKey = Guid.NewGuid().ToString(),
    SourceSymbol = "USD",
    DestinationSymbol = "USDC",
};
var response = transactionsService.CreateConversion(request);

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
transactionsService := transactions.NewTransactionsService(client)
request := &transactions.CreateConversionRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    WalletId: "WALLET_ID_HERE",
    Amount: "1",
    Destination: "DESTINATION_WALLET_UUID",
    IdempotencyKey: uuid.New().String(),
    SourceSymbol: "USD",
    DestinationSymbol: "USDC",
}
response, err := transactionsService.CreateConversion(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = CreateConversionRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
    amount = '1',
    destination = 'DESTINATION_WALLET_UUID',
    idempotency_key = str(uuid.uuid4()),
    source_symbol = 'USD',
    destination_symbol = 'USDC',
)
response = prime_client.create_conversion(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl create-conversion --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const transactionsService = new TransactionsService(client);
transactionsService.createConversion({
    portfolioId: 'PORTFOLIO_ID_HERE',
    walletId: 'WALLET_ID_HERE',
    amount: "1",
    destination: "DESTINATION_WALLET_UUID",
    idempotencyKey: uuidv4(),
    sourceSymbol: "USD",
    destinationSymbol: "USDC",
}).then(async (response) => {
    console.log('Conversion: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Path Parameters

The wallet ID that the conversion will originate from

#### Body

The amount in whole units to convert

The UUID of the destination wallet

Example:

`"e84255eb-2e21-439e-a1d0-f5dd1e1292b9"`

The idempotency key associated with this conversion

Example:

`"e84255eb-2e21-439e-a1d0-f5dd1e1292b9"`

The currency symbol to convert from

The currency symbol to convert to

#### Response

The activity ID for the conversion

Example:

`"e84255eb-2e21-439e-a1d0-f5dd1e1292b9"`

The currency symbol to convert from

The currency symbol to convert to

The amount in whole units to convert

The UUID of the destination wallet

Example:

`"e84255eb-2e21-439e-a1d0-f5dd1e1292b9"`

The UUID of the source wallet

Example:

`"e84255eb-2e21-439e-a1d0-f5dd1e1292b9"`

The UUID of the conversion transaction

Example:

`"e84255eb-2e21-439e-a1d0-f5dd1e1292b9"`