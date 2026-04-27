# get entity payment method

Get Entity Payment Method

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
PaymentMethodsService paymentMethodsService = PrimeServiceFactory.createPaymentMethodsService(client);
GetEntityPaymentMethodRequest request = new GetEntityPaymentMethodRequest.Builder()
    .entityId("ENTITY_ID_HERE")
    .paymentMethodId("PAYMENT_METHOD_ID_HERE")
    .build();
GetEntityPaymentMethodResponse response = paymentMethodsService.getEntityPaymentMethod(request);

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var paymentMethodsService = new PaymentMethodsService(client);
var request = new GetEntityPaymentMethodRequest("ENTITY_ID_HERE", "PAYMENT_METHOD_ID_HERE");
var response = paymentMethodsService.GetEntityPaymentMethod(request);

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
paymentMethodsService := paymentmethods.NewPaymentMethodsService(client)
request := &paymentmethods.GetEntityPaymentMethodRequest{
    Id: "ENTITY_ID_HERE",
    PaymentMethodId: "PAYMENT_METHOD_ID_HERE",
}
response, err := paymentMethodsService.GetEntityPaymentMethod(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = GetEntityPaymentMethodRequest(
    entity_id="ENTITY_ID_HERE",
    payment_method_id="PAYMENT_METHOD_ID_HERE",
)
response = prime_client.get_entity_payment_method(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl get-entity-payment-method --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const paymentMethodsService = new PaymentMethodsService(client);
paymentMethodsService.getPaymentMethod({
    entityId: 'ENTITY_ID_HERE',
    paymentMethodId: 'PAYMENT_METHOD_ID_HERE'
}).then(async (response) => {
    console.log('Payment Methods: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Path Parameters

#### Response