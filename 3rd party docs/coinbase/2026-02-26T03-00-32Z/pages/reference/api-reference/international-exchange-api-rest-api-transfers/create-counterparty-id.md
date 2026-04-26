# create counterparty id

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
TransfersService transfersService = IntxServiceFactory.createTransfersService(client);
CreateCounterpartyIdRequest request = new CreateCounterpartyIdRequest.Builder()
    .portfolio("portfolio_id")
    .build();
CreateCounterpartyIdResponse response = transfersService.createCounterpartyId(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var transfersService = new TransfersService(client);
var request = new CreateCounterpartyIdRequest(
    Portfolio: "portfolio_id",
);
var response = transfersService.CreateCounterpartyId(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
transfersSvc := transfers.NewTransfersService(client)
request := &transfers.CreateCounterpartyIdRequest{
    Portfolio: "portfolio_id",
}
response, err := transfersSvc.CreateCounterpartyId(context.Background(), request)

```

For more information, please visit the [INTX Go SDK](https://github.com/coinbase-samples/intx-sdk-go).

```
client = IntxClient()
request = CreateCounterpartyIdRequest(
    portfolio="portfolio_id",
)
response = client.create_counterparty_id(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const transfersService = new TransfersService(client);
transfersService.createCounterparty({
    portfolio: 'PORTFOLIO_ID_HERE',
}).then(async (response) => {
    console.log('Counterparty created: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

```
intxctl create-counterparty-id --help

```

For more information, please visit the [INTX CLI](https://github.com/coinbase-samples/intx-cli).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Body

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`)

#### Response

UUID for the portfolio

Example:

`"3d50e347-6a59-4965-a4cd-b25934d84126"`

Counterparty Id for the portfolio