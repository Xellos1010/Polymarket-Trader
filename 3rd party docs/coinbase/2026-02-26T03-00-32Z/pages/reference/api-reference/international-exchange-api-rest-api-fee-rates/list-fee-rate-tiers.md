# list fee rate tiers

-   Java
    
-   .NET
    
-   Python
    
-   TS/JS
    

```
FeeRatesService feeRatesService = IntxServiceFactory.createFeeRatesService(client);
GetFeeRateTiersResponse response = feeRatesService.getFeeRateTiers();

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var feeRatesService = new FeeRatesService(client);
var response = feeRatesService.GetFeeRateTiers();

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
client = IntxClient()
request = ListFeeRateTiersRequest()
response = client.list_fee_rate_tiers(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const feeRatesService = new FeeRatesService(client);
feeRatesService.listFeeRateTiers().then(async (response) => {
    console.log('Fee Rate Tiers: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

#### Response

The type of fee tier

Available options

:

`REGULAR`,

`LIQUIDITY_PROGRAM`

Example:

`"LIQUIDITY_PROGRAM"`

The type of instrument

Available options

:

`SPOT`,

`PERP`

Name for the fee rate tier

The fee rate charged for order making liquidity

The fee rate charged for orders taking liquidity

The minimum USDC balance required to qualify for the fee tier

The minimum 30 days volume required to qualify for the fee tier. For liquidity program tiers, this is the percentage of volume

require\_balance\_and\_volume

Whether the fee tier requires both a minimum balance and volume to qualify