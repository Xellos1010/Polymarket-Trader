# get block trade position limit

Get Block Trade Position Limit

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade-position-limits/{uuid}
```

```
{
  "position_limit_uuid": "123e4567-e89b-12d3-a456-426614174000",
  "account_uuid": "abcdef12-3456-7890-abcd-ef1234567890",
  "symbol": "BIPZ30",
  "position_limit": 1000,
  "max_order_size": 100
}
```

Retrieves information for a block trade position limit with the specified UUID.

GET

/

rest

/

block-trade-position-limits

/

{uuid}

Get Block Trade Position Limit

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade-position-limits/{uuid}
```

```
{
  "position_limit_uuid": "123e4567-e89b-12d3-a456-426614174000",
  "account_uuid": "abcdef12-3456-7890-abcd-ef1234567890",
  "symbol": "BIPZ30",
  "position_limit": 1000,
  "max_order_size": 100
}
```

#### Path Parameters

The block-trade-position-limit's UUID

Example:

`"e80d6a4e-af9f-4fcb-a819-3d22c7017279"`

#### Response

Successfully retrieved block trade position limit

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Example:

`"abcdef12-3456-7890-abcd-ef1234567890"`