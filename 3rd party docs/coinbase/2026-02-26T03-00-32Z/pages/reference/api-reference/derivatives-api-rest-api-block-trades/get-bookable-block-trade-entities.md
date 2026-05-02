# get bookable block trade entities

Get Bookable Block Trade Entities

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade/booking/bookable-entities
```

```
[
  {
    "entity_uuid": "123e4567-e89b-12d3-a456-426614174000",
    "name": "Block Trade Entity 1",
    "code": "BTE1",
    "is_permitted_broker": false,
    "admin_trading_lock": false,
    "client_requested_trading_lock": false,
    "firm_uuid": "abcdef12-3456-7890-abcd-ef1234567890"
  }
]
```

Retrieves the list of block trade entities that the authenticated user can book trades for.

GET

/

rest

/

block-trade

/

booking

/

bookable-entities

Get Bookable Block Trade Entities

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade/booking/bookable-entities
```

```
[
  {
    "entity_uuid": "123e4567-e89b-12d3-a456-426614174000",
    "name": "Block Trade Entity 1",
    "code": "BTE1",
    "is_permitted_broker": false,
    "admin_trading_lock": false,
    "client_requested_trading_lock": false,
    "firm_uuid": "abcdef12-3456-7890-abcd-ef1234567890"
  }
]
```

#### Response

Successfully retrieved bookable entities

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Example:

`"Block Trade Entity 1"`

client\_requested\_trading\_lock

Example:

`"abcdef12-3456-7890-abcd-ef1234567890"`