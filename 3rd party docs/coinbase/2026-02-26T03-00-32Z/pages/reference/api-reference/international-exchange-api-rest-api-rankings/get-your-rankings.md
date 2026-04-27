# get your rankings

-   TS/JS
    

```
const rankingsService = new RankingsService(client);
rankingsService.getRankings().then(async (response) => {
    console.log('Volume Rankings: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Query Parameters

Identifies the instruments by type . Allowed values: SPOT, PERPETUAL\_FUTURE

Available options

:

`SPOT`,

`PERPETUAL_FUTURE`

Identifies the lookback window for the query . Allowed values: YESTERDAY, LAST\_7\_DAYS, THIS\_MONTH, LAST\_30\_DAYS, LAST\_MONTH. Default: THIS\_MONTH

Available options

:

`YESTERDAY`,

`LAST_7_DAYS`,

`THIS_MONTH`,

`LAST_30_DAYS`,

`LAST_MONTH`

One or more instrument identifiers, such as name (e.g., `BTC-USDC`), UUID (e.g., `ce55a827-f04a-45c0-9d9b-8bbdb9b48065`), or instrument ID (e.g., `7149252043835013`). If not provided, the query will return the rankings for all instruments of the specified type. If one or more instruments are specified, the query will return the volume and relative percent for all specified instruments, but the returned rank will be "--".

#### Response

The time of the most recent status change of the statistics

Example:

`"2023-01-29T14:32:28.000Z"`