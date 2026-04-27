# validate counterparty id

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
TransfersService transfersService = IntxServiceFactory.createTransfersService(client);
ValidateCounterpartyIdRequest request = new ValidateCounterpartyIdRequest.Builder()
    .counterpartyId("counterparty_id")
    .build();
ValidateCounterpartyIdResponse response = transfersService.validateCounterpartyId(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var transfersService = new TransfersService(client);
var request = new ValidateCounterpartyIdRequest(
    CounterpartyId: "counterparty_id",
);
var response = transfersService.ValidateCounterpartyId(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
transfersSvc := transfers.NewTransfersService(client)
request := &transfers.ValidateCounterpartyIdRequest{
    CounterpartyId: "counterparty_id",
}
response, err := transfersSvc.ValidateCounterpartyId(context.Background(), request)

```

For more information, please visit the [INTX Go SDK](https://github.com/coinbase-samples/intx-sdk-go).

```
client = IntxClient()
request = ValidateCounterpartyIdRequest(
    counterparty_id="counterparty_id",
)
response = client.validate_counterparty_id(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const transfersService = new TransfersService(client);
transfersService.validateCounterparty({
    counterpartyId: 'COUNTERPARTY_ID_HERE',
}).then(async (response) => {
    console.log('Counterparty created: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

```
intxctl validate-counterparty-id --help

```

For more information, please visit the [INTX CLI](https://github.com/coinbase-samples/intx-cli).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Body

Counterparty Id to be validated

#### Response

Return whether the counterparty Id is valid to withdraw to

Whether the counterparty Id is valid to withdraw to