# get all block trade entities

Get All Block Trade Entities

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade-entities
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

Retrieves a list of all block trade entities you have access to.

GET

/

rest

/

block-trade-entities

Get All Block Trade Entities

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade-entities
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

Successfully retrieved block trade entities

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Example:

`"Block Trade Entity 1"`

client\_requested\_trading\_lock

Example:

`"abcdef12-3456-7890-abcd-ef1234567890"`