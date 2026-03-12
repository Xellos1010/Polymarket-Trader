# get index candles

-   TS/JS
    

```
const indexService = new IndexService(client);
indexService.getIndexCandles({
    index: 'COIN50',
    granularity: 'ONE_DAY',
    start: '2024-01-01T00:00:00Z',
}).then(async (response) => {
    console.log('Index Candles: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Path Parameters

Identifies the index by name (e.g., `COIN50`)

#### Query Parameters

The aggregation period of the candles data

Available options

:

`ONE_DAY`,

`ONE_HOUR`

Start timestamp in ISO 8601 timestamp format (e.g. `2024-10-21T00:00:00Z`)

End timestamp in ISO 8601 timestamp format (e.g. `2024-10-31T00:00:00Z`)

#### Response