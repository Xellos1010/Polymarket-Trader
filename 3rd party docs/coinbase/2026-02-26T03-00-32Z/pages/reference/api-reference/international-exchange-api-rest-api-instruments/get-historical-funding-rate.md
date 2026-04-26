# get historical funding rate

Get historical funding rates

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
InstrumentsService instrumentsService = IntxServiceFactory.createInstrumentsService(client);
GetHistoricalFundingRatesRequest request = new GetHistoricalFundingRatesRequest.Builder()
    .instrumentId("BTC-PERP")
    .build();
GetHistoricalFundingRatesResponse response = instrumentsService.getHistoricalFundingRates(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var instrumentsService = new InstrumentsService(client);
var request = new GetHistoricalFundingRatesRequest(
    InstrumentId: "BTC-PERP",
);
var response = instrumentsService.GetHistoricalFundingRates(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
instrumentsSvc := instruments.NewInstrumentsService(client)
request := &instruments.GetHistoricalFundingRequest{
    InstrumentId: "BTC-PERP",
}
response, err := instrumentsSvc.GetHistoricalFunding(context.Background(), request)

```

For more information, please visit the [INTX Go SDK](https://github.com/coinbase-samples/intx-sdk-go).

```
client = IntxClient()
request = GetHistoricalFundingRatesRequest(
    instrument_id="BTC-PERP",
)
response = client.get_historical_funding_rates(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const instrumentsService = new InstrumentsService(client);
instrumentsService.getHistoricalFundingRates({
    instrument: 'ETH-PERP',
}).then(async (response) => {
    console.log('Historical Funding Rates: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

```
intxctl get-historical-funding-rates --help

```

For more information, please visit the [INTX CLI](https://github.com/coinbase-samples/intx-cli).

#### Path Parameters

Identifies the instrument by name (e.g., `BTC-PERP`), UUID (e.g., `ce55a827-f04a-45c0-9d9b-8bbdb9b48065`), or instrument ID (e.g., `7149252043835013`)

#### Query Parameters

The number of results to return (defaults to 25 with a max supported value of 100)

The number of results from the beginning to skip past

#### Response

The unique identifier of the instrument for which the funding rate applies

The final funding rate based on the state of the rolling calculation at the `event_time`.

The current mark price value used in risk and margin calculations

The time that the final funding rate was determined. Uses ISO-8601 format (e.g., 2023-03-16T23:59:53Z)

Example:

`"2023-03-16T23:59:53.000Z"`