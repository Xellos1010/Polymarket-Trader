# create portfolio allocations

Create Portfolio Allocations

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/allocations \
  --header 'Content-Type: application/json' \
  --data '
{
  "allocation_id": "<string>",
  "source_portfolio_id": "<string>",
  "product_id": "<string>",
  "order_ids": [
    "<string>"
  ],
  "allocation_legs": [
    {
      "allocation_leg_id": "<string>",
      "destination_portfolio_id": "<string>",
      "amount": "<string>"
    }
  ],
  "size_type": "BASE",
  "remainder_destination_portfolio": "<string>"
}
'
```

```
{
  "body": {
    "success": true,
    "allocation_id": "<string>",
    "failure_reason": "<string>"
  }
}
```

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
AllocationsService allocationsService = PrimeServiceFactory.createAllocationsService(client);
String allocationId = UUID.randomUUID().toString();
String allocationLegId = UUID.randomUUID().toString();
AllocationLeg allocationLeg = new AllocationLeg.Builder()
        .allocationLegId(allocationLegId)
        .amount("100")
        .destinationPortfolioId("DESTINATION_PORTFOLIO_ID_HERE")
        .build();
CreateAllocationRequest request = new CreateAllocationRequest.Builder()
        .sourcePortfolioId("SOURCE_PORTFOLIO_ID_HERE")
        .allocationId(allocationId)
        .allocationLegs(new AllocationLeg[]{allocationLeg})
        .productId("ETH-USD")
        .build();
CreateAllocationResponse response = allocationsService.createAllocation(request);

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var allocationsService = new AllocationsService(client);
var allocationId = Guid.NewGuid();
var allocationLegId = Guid.NewGuid();
var allocationLeg = new AllocationLeg()
{
    AllocationLegId = allocationLegId.ToString(),
    Amount = "100",
    DestinationPortfolioId = "ADD_DESTINATION_PORTFOLIO_ID_HERE",
};
var request = new CreateAllocationRequest()
{
    AllocationId = allocationId.ToString(),
    ProductId = "ETH-USD",
    SourcePortfolioId = "ADD_SOURCE_PORTFOLIO_ID_HERE",
    AllocationLegs = [ allocationLeg ],
    SizeType = Prime.Model.SizeType.PERCENT,
};
var response = allocationsService.CreateAllocation(request);

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
allocationsService := allocations.NewAllocationsService(client)
allocationId := uuid.New().String()
allocationLegId := uuid.New().String()
allocationLeg := &model.AllocationLeg{
    LegId:                  allocationLegId,
    DestinationPortfolioId: "DESTINATION_PORTFOLIO_ID_GOES_HERE",
    Amount:                 "100.0",
}
request := &allocations.CreatePortfolioAllocationsRequest{
    AllocationId:      allocationId,
    SourcePortfolioId: "SOURCE_PORTFOLIO_ID_GOES_HERE",
    ProductId:         "ETH-USD",
    AllocationLegs:    []*model.AllocationLeg{allocationLeg},
    OrderIds:          []string{"ORDER_IDS_TO_BE_ALLOCATED_HERE"},
    SizeType:          "PERCENT",
}
response, err := allocationsService.CreatePortfolioAllocations(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
allocation_id = uuid.uuid4()
allocation_leg_id = uuid.uuid4()
product_id = 'ETH-USD'
size_type = 'PERCENT'
allocation_leg = AllocationLeg(
    leg_id=allocation_leg_id,
    destination_portfolio_id='DESTINATION_PORTFOLIO_ID_GOES_HERE',
    amount='100.0',
)
request = CreatePortfolioAllocationsRequest(
    allocation_id=allocation_id,
    source_portfolio_id='SOURCE_PORTFOLIO_ID_GOES_HERE',
    product_id=product_id,
    order_ids=['ORDER_ID_GOES_HERE'],
    allocation_legs=[allocation_leg],
    size_type=size_type,
)
response = prime_client.create_portfolio_allocations(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl create-allocation --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const allocationService = new AllocationService(client);
allocationService.createAllocation({
    allocationId: uuidv4(),
    sourcePortfolioId: "SOURCE_PORTFOLIO_ID_GOES_HERE"
    productId: "ETH-USD",
    orderIds: ["ORDER_ID_GOES_HERE"],
    allocationLegs: [{
        legId:                  uuidv4(),
        destinationPortfolioId: "DESTINATION_PORTFOLIO_ID_GOES_HERE",
        amount:                 "100.0",
    }]
    sizeType: AllocationSizeType.Percent
}).then(async (response) => {
    console.log('Order allocated: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Body

The source portfolio id for the allocation

The product for the allocation

The list of order ids in the allocation

The list of allocation\_legs for the allocation

Available options

:

`BASE`,

`QUOTE`,

`PERCENT`

remainder\_destination\_portfolio

The portfolio where to allocate the remainder of the size

#### Response