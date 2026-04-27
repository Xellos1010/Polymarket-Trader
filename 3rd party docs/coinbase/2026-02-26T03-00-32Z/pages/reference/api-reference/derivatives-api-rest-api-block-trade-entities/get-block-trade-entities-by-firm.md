# get block trade entities by firm

Get Block Trade Entities by Firm

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/firm-block-trade-entities/{firm_uuid}
```

```
{
  "entity_uuid": "123e4567-e89b-12d3-a456-426614174000",
  "name": "Block Trade Entity 1",
  "code": "BTE1",
  "is_permitted_broker": false,
  "admin_trading_lock": false,
  "client_requested_trading_lock": false,
  "firm_uuid": "abcdef12-3456-7890-abcd-ef1234567890"
}
```

Retrieves the block trade entity associated with the specified firm.

GET

/

rest

/

firm-block-trade-entities

/

{firm\_uuid}

Get Block Trade Entities by Firm

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/firm-block-trade-entities/{firm_uuid}
```

```
{
  "entity_uuid": "123e4567-e89b-12d3-a456-426614174000",
  "name": "Block Trade Entity 1",
  "code": "BTE1",
  "is_permitted_broker": false,
  "admin_trading_lock": false,
  "client_requested_trading_lock": false,
  "firm_uuid": "abcdef12-3456-7890-abcd-ef1234567890"
}
```

#### Path Parameters

The firm's UUID

Example:

`"e80d6a4e-af9f-4fcb-a819-3d22c7017279"`

#### Response

Successfully retrieved block trade entities for the firm

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Example:

`"Block Trade Entity 1"`

client\_requested\_trading\_lock

Example:

`"abcdef12-3456-7890-abcd-ef1234567890"`