# list position offsets

```
curl --request GET \
  --url https://api.international.coinbase.com/api/v1/position-offsets
```

```
{
  "position_offsets": [
    {
      "primary_instrument_id": "14thr7ft-1-0",
      "secondary_instrument_id": "31bpa1qz-1-1",
      "offset": 0.02
    }
  ]
}
```

Returns all active position offsets

```
curl --request GET \
  --url https://api.international.coinbase.com/api/v1/position-offsets
```

```
{
  "position_offsets": [
    {
      "primary_instrument_id": "14thr7ft-1-0",
      "secondary_instrument_id": "31bpa1qz-1-1",
      "offset": 0.02
    }
  ]
}
```

-   TS/JS
    

```
const positionOffsetsService = new PositionOffsetsService(client);
positionOffsetsService.listPositionOffsets().then(async (response) => {
    console.log('Position Offsets: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

#### Response