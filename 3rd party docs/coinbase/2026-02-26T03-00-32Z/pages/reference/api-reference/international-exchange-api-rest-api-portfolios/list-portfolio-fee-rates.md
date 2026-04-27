# list portfolio fee rates

-   Java
    
-   .NET
    
-   Python
    
-   TS/JS
    

```
PortfoliosService portfoliosService = IntxServiceFactory.createPortfoliosService(client);
ListPortfolioFeeRatesResponse response = portfoliosService.listPortfolioFeeRates();

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var portfoliosService = new PortfoliosService(client);
var response = portfoliosService.ListPortfolioFeeRates();

```

```
client = IntxClient()
request = ListPortfolioFeeRatesRequest()
response = client.list_portfolio_fee_rates(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.listPortfolioFeeRates().then(async (response) => {
    console.log('Portfolio Fee Rates: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Response

The type of instrument

Available options

:

`SPOT`,

`PERP`

Whether the fee tier is a VIP tier

Name for the fee rate tier

The fee rate charged for order making liquidity

The fee rate charged for orders taking liquidity

Whether maker/taker fee rates are manually overridden

Sum of trading volume from last rolling 30 days

trailing\_24hr\_usdc\_balance

Average of all hourly USDC balance snapshots from the last rolling 24 hours