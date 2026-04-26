# transfer funds

Transfer funds between portfolios

## API Key Permissions

This endpoint requires an API key with `transfer` permission.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
PortfoliosService portfoliosService = IntxServiceFactory.createPortfoliosService(client);
TransferFundsRequest request = new TransferFundsRequest.Builder()
    .from("portfolio_id_1")
    .to("portfolio_id_2")
    .asset("BTC")
    .amount("1")
    .build();
TransferFundsResponse response = portfoliosService.transferFunds(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var portfoliosService = new PortfoliosService(client);
var request = new TransferFundsRequest(
    From: "portfolio_id_1",
    To: "portfolio_id_2",
    Asset: "BTC",
    Amount: "1",
);
var response = portfoliosService.TransferFunds(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
ordersSvc := orders.NewOrdersService(client)
request := &portfolios.CreatePortfolioTransferRequest{
    From: "portfolio_id_1",
    To: "portfolio_id_2",
    Asset: "BTC",
    Amount: "1",
}
response, err := portfoliosSvc.CreatePortfolioTransfer(context.Background(), request)

```

For more information, please visit the [INTX Go SDK](https://github.com/coinbase-samples/intx-sdk-go).

```
client = IntxClient()
request = TransferFundsRequest(
    from="portfolio_id_1",
    to="portfolio_id_2",
    asset="BTC",
    amount="1",
)
response = client.transfer_funds(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.createTransferFunds({
    from: 'portfolio_id_1',
    to: 'portfolio_id_2',
    asset: 'ETH',
    amount: '1',
}).then(async (response) => {
    console.log('Transfer Created: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

```
intxctl create-portfolio-transfer --help

```

For more information, please visit the [INTX CLI](https://github.com/coinbase-samples/intx-cli).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Body

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`) to transfer funds from

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`) to transfer funds to

Identifies the asset by name (e.g., `BTC`), UUID (e.g., `291efb0f-2396-4d41-ad03-db3b2311cb2c`), or asset ID (e.g., `1482439423963469`)

The amount of the asset being transferred

#### Response

true if the transfer was successful