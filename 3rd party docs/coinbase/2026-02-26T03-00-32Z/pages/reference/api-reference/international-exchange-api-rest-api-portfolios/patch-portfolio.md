# patch portfolio

-   Java
    
-   .NET
    
-   Python
    

```
PortfoliosService portfoliosService = IntxServiceFactory.createPortfoliosService(client);
PatchPortfolioRequest request = new PatchPortfolioRequest.Builder().build();
PatchPortfolioResponse response = portfoliosService.patchPortfolio(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var portfoliosService = new PortfoliosService(client);
var request = new PatchPortfolioRequest();
var response = portfoliosService.PatchPortfolio(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
client = IntxClient()
request = PatchPortfolioRequest()
response = client.patch_portfolio(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Path Parameters

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`)

#### Body

Whether auto margin is enabled for the portfolio

Whether cross collateral is enabled for the portfolio

Whether position offsets is enabled for the portfolio

pre\_launch\_trading\_enabled

Whether pre-launch trading is enabled for the portfolio

Whether margin call is enabled for the portfolio

disable\_overdraft\_protection

Disables loan overdraft protection for the portfolio

Display name for portfolio

#### Response

A unique identifier for the portfolio

A UUID for the portfolio

Example:

`"3d50e347-6a59-4965-a4cd-b25934d84126"`

A human readable name for the portfolio

Example:

`"Investment Account"`

A user UUID for brokers that attribute a single user per portfolio

Example:

`"f67de785-60a7-45ea-b87a-07e83eae7c12"`

The fee rate charged for order making liquidity

The fee rate charged for orders taking liquidity

Indicates if the portfolio has been locked from trading

Indicates whether or not the portfolio can borrow

Indicates if the portfolio is setup to take liquidation assignments

Indicates if the portfolio is the account default portfolio

Indicates if the cross collateral is enabled for the portfolio

pre\_launch\_trading\_enabled

Indicates if pre-launch trading is enabled for the portfolio

disable\_overdraft\_protection

Indicates if loan overdraft protection is disabled for the portfolio