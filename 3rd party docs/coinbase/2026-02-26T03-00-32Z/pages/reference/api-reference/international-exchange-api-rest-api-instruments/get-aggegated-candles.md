# get aggegated candles

Get aggregated candles data per instrument

-   Java
    
-   .NET
    
-   Python
    
-   TS/JS
    

```
InstrumentsService instrumentsService = IntxServiceFactory.createInstrumentsService(client);
GetAggregatedCandlesRequest request = new GetAggregatedCandlesRequest.Builder()
    .instrumentId("BTC-PERP")
    .granularity("ONE_DAY")
    .start("2024-01-01T00:00:00Z")
    .build();
GetAggregatedCandlesResponse response = instrumentsService.getAggregatedCandles(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var instrumentsService = new InstrumentsService(client);
var request = new GetAggregatedCandlesRequest(
    InstrumentId: "BTC-PERP",
    Granularity: "ONE_DAY",
    Start: "2024-01-01T00:00:00Z",
);
var response = instrumentsService.GetAggregatedCandles(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
client = IntxClient()
request = GetAggregatedCandlesRequest(
    instrument_id="BTC-PERP",
    granularity="ONE_DAY",
    start="2024-01-01T00:00:00Z",
)
response = client.get_aggregated_candles(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const instrumentsService = new InstrumentsService(client);
instrumentsService.getAggregatedCandles({
    instrumentId: 'BTC-PERP',
    granularity: 'ONE_DAY',
    start: '2024-01-01T00:00:00Z',
}).then(async (response) => {
    console.log('Aggregated Candles: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

#### Path Parameters

Identifies the instrument by name (e.g., `BTC-PERP`)

#### Query Parameters

The aggregation period of the candles data. End timestamp in ISO 8601 timestamp format (e.g. 2024-03-01T00:00:00Z).

Available options

:

`ONE_DAY`,

`SIX_HOUR`,

`TWO_HOUR`,

`ONE_HOUR`,

`THIRTY_MINUTE`,

`FIFTEEN_MINUTE`,

`FIVE_MINUTE`,

`ONE_MINUTE`

Start timestamp in ISO 8601 timestamp format (e.g. `2024-03-01T00:00:00Z`)

End timestamp in ISO 8601 timestamp format (e.g. `2024-03-01T00:00:00Z`)

#### Response