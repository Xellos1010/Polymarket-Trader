# get bookable block traders

Get Bookable Block Traders

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade/booking/bookable-traders
```

```
[
  {
    "trader_uuid": "123e4567-e89b-12d3-a456-426614174000",
    "name": "trader1",
    "full_name": "John Doe",
    "block_trade_entity_uuid": "abcdef12-3456-7890-abcd-ef1234567890",
    "email": "trader@example.com"
  }
]
```

Retrieves the list of block traders that the authenticated user can book trades for.

GET

/

rest

/

block-trade

/

booking

/

bookable-traders

Get Bookable Block Traders

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade/booking/bookable-traders
```

```
[
  {
    "trader_uuid": "123e4567-e89b-12d3-a456-426614174000",
    "name": "trader1",
    "full_name": "John Doe",
    "block_trade_entity_uuid": "abcdef12-3456-7890-abcd-ef1234567890",
    "email": "trader@example.com"
  }
]
```

#### Response

Successfully retrieved bookable traders

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Example:

`"abcdef12-3456-7890-abcd-ef1234567890"`

Example:

`"trader@example.com"`