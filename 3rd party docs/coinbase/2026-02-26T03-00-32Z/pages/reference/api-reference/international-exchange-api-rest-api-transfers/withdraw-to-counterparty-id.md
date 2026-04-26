# withdraw to counterparty id

Withdraw to counterparty Id

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
TransfersService transfersService = IntxServiceFactory.createTransfersService(client);
WithdrawToCounterpartyIdRequest request = new WithdrawToCounterpartyIdRequest.Builder()
    .portfolio("portfolio_id")
    .counterpartyId("counterparty_id")
    .asset("BTC")
    .amount("1")
    .build();
WithdrawToCounterpartyIdResponse response = transfersService.withdrawToCounterpartyId(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var transfersService = new TransfersService(client);
var request = new WithdrawToCounterpartyIdRequest(
    Portfolio: "portfolio_id",
    CounterpartyId: "counterparty_id",
    Asset: "BTC",
    Amount: "1",
);
var response = transfersService.WithdrawToCounterpartyId(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
transfersSvc := transfers.NewTransfersService(client)
request := &transfers.CreateWithdrawalToCounterpartyIdRequest{
    Portfolio: "portfolio_id",
    CounterpartyId: "counterparty_id",
    Asset: "BTC",
    Amount: "1",
}
response, err := transfersSvc.CreateWithdrawalToCounterpartyId(context.Background(), request)

```

For more information, please visit the [INTX Go SDK](https://github.com/coinbase-samples/intx-sdk-go).

```
client = IntxClient()
request = WithdrawToCounterpartyIdRequest(
    portfolio="portfolio_id",
    counterparty_id="counterparty_id",
    asset="BTC",
    amount="1",
)
response = client.withdraw_to_counterparty_id(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const transfersService = new TransfersService(client);
transfersService.withdrawToCounterparty({
    portfolio: 'PORTFOLIO_ID_HERE',
    counterpartyId: 'COUNTERPARTY_ID_HERE',
    asset: 'BTC',
    amount: "1",
    nonce: 12345
}).then(async (response) => {
    console.log('Withdrawal created: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

```
intxctl create-withdrawal-to-counterparty-id --help

```

For more information, please visit the [INTX CLI](https://github.com/coinbase-samples/intx-cli).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Body

Identifies the portfolio to withdraw from by UUID (e.g., `892e8c7c-e979-4cad-b61b-55a197932cf1`) or portfolio ID (e.g., `5189861793641175`)

Counterparty Id to withdraw to

Identifies the asset by name (e.g., `BTC`), UUID (e.g., `291efb0f-2396-4d41-ad03-db3b2311cb2c`), or asset ID (e.g., `1482439423963469`)

The amount of the asset being transferred

A unique positive integer representing the withdrawal request

#### Response

Counterparty withdrawal initiated

Idempotent UUID representing the successful withdraw

Example:

`"253b30a5-9b03-4cd2-9c76-d0f32b2bd733"`

UUID for the portfolio where the withdraw was initiated

Example:

`"3d50e347-6a59-4965-a4cd-b25934d84126"`

Counterparty Id of the source portfolio

Counterparty Id of the target portfolio

The asset being transferred

The amount of the asset being transferred