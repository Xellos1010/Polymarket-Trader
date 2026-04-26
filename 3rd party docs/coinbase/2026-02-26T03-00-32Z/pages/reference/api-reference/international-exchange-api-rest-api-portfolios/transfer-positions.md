# transfer positions

Transfer positions between portfolios

## API Key Permissions

This endpoint requires an API key with `trade` permission.

-   Java
    
-   .NET
    
-   Python
    
-   TS/JS
    

```
PortfoliosService portfoliosService = IntxServiceFactory.createPortfoliosService(client);
TransferPositionsRequest request = new TransferPositionsRequest.Builder()
    .from("portfolio_id_1")
    .to("portfolio_id_2")
    .instrument("BTC-PERP")
    .amount("1")
    .build();
TransferPositionsResponse response = portfoliosService.transferPositions(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var portfoliosService = new PortfoliosService(client);
var request = new TransferPositionsRequest(
    From: "portfolio_id_1",
    To: "portfolio_id_2",
    Instrument: "BTC-PERP",
    Amount: "1",
);
var response = portfoliosService.TransferPositions(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
client = IntxClient()
request = TransferPositionRequest(
    from="portfolio_id_1",
    to="portfolio_id_2",
    instrument="BTC-PERP",
    amount="1",
)
response = client.transfer_positions(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const portfoliosService = new PortfoliosService(client);
portfoliosService.createTransferPosition({
    from: 'portfolio_id_1',
    to: 'portfolio_id_2',
    instrument: 'ETH-PERP',
    amount: '1',
}).then(async (response) => {
    console.log('Transfer Created: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Body

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`) to transfer positions from

Identifies the portfolio by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`) to transfer positions to

Identifies the instrument by name (e.g., `BTC-PERP`), UUID (e.g., `291efb0f-2396-4d41-ad03-db3b2311cb2c`), or instrument ID (e.g., `1482439423963469`)

The full or partial quantity of the position being transferred

The side of the position being transferred, BUY or SELL

#### Response

true if the transfer was successful