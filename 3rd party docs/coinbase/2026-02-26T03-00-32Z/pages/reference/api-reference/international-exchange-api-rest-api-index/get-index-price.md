# get index price

-   TS/JS
    

```
const indexService = new IndexService(client);
indexService.getIndexPrice({
    index: 'COIN50',
}).then(async (response) => {
    console.log('Index Price: ', response);
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

#### Response

The Product ID for the index

The status of the index price

The timestamp of the index price

Example:

`"2024-11-11T20:42:33Z"`

The latest index price

Example:

`"388.91871799439963"`

The price 24hr change percent (negative values denote change down)

Example:

`"8.940862872710408"`