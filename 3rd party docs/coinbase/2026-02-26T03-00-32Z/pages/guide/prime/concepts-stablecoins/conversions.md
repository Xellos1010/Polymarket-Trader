# conversions

USDC and PYUSD on Coinbase Prime are acquired through [Create Conversion](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-conversion), which bypasses the order book entirely. Conversions are instant, free, 1:1, and bidirectional, allowing movement between USD and USDC or USD and PYUSD in either direction.

## Creating a Stablecoin Conversion

The following examples show how to create a conversion between USD and USDC.

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
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.transactions import TransactionsService, CreateConversionRequest
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
transactions_service = TransactionsService(client)
  
request = CreateConversionRequest(  
    portfolio_id="PORTFOLIO_ID_HERE",  
    wallet_id="WALLET_ID_HERE",  
    amount = '1',  
    destination = 'DESTINATION_WALLET_UUID',  
    idempotency_key = str(uuid.uuid4()),  
    source_symbol = 'USD',  
    destination_symbol = 'USDC',  
)  
  
response = transactions_service.create_conversion(request)  

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
 primectl create-conversion --help  

```

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

## Tracking a Stablecoin Conversion

Conversions are tracked via [Get Transaction by ID](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/get-transaction-by-transaction-id). The process is identical to tracking a withdrawal. Simply look up the transaction by its ID. Typically, conversion transactions reach a terminal state within a few seconds.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```

TransactionsService transactionsService = PrimeServiceFactory.createTransactionsService(client);  
  
GetTransactionByTransactionIdRequest request = new GetTransactionByTransactionIdRequest.Builder()  
    .portfolioId("PORTFOLIO_ID_HERE")  
    .transactionId("TRANSACTION_ID_HERE")  
    .build();  
  
GetTransactionByTransactionIdResponse response = transactionsService.getTransactionByTransactionId(request);  

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```

var transactionsService = new TransactionsService(client);  
  
var request = new GetTransactionByTransactionIdRequest("PORTFOLIO_ID_HERE", "TRANSACTION_ID_HERE");  
  
var response = transactionsService.GetPortfolioById(request);  

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
transactionsService := transactions.NewTransactionsService(client)  
  
request := &transactions.GetPortfolio{  
    PortfolioId: "PORTFOLIO_ID_HERE",  
    TransactionId: "TRANSACTION_ID_HERE",  
}  
  
response, err := transactionsService.GetTransactions(context.Background(), request)  

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.transactions import TransactionsService, GetTransactionByTransactionIdRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
transactions_service = TransactionsService(client)
  
request = GetTransactionRequest(
    portfolio_id="PORTFOLIO_ID_HERE",  
    transaction_id="TRANSACTION_ID_HERE",  
)  
  
response = transactions_service.get_transaction(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```

const transactionsService = new TransactionsService(client);  
  
transactionsService.getTransaction({  
    portfolioId: 'PORTFOLIO_ID_HERE',  
    transactionId: 'TRANSACTION_ID_HERE',  
}).then(async (response) => {  
    console.log('Transaction: ', response);  
})  

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl get-transaction --help  

```