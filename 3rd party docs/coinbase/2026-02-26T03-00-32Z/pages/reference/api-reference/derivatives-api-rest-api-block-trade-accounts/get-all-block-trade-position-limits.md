# get all block trade position limits

Get All Block Trade Position Limits

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade-position-limits
```

```
[
  {
    "position_limit_uuid": "123e4567-e89b-12d3-a456-426614174000",
    "account_uuid": "abcdef12-3456-7890-abcd-ef1234567890",
    "symbol": "BIPZ30",
    "position_limit": 1000,
    "max_order_size": 100
  }
]
```

Retrieves a list of all block trade position limits you have access to.

GET

/

rest

/

block-trade-position-limits

Get All Block Trade Position Limits

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade-position-limits
```

```
[
  {
    "position_limit_uuid": "123e4567-e89b-12d3-a456-426614174000",
    "account_uuid": "abcdef12-3456-7890-abcd-ef1234567890",
    "symbol": "BIPZ30",
    "position_limit": 1000,
    "max_order_size": 100
  }
]
```

#### Response

Successfully retrieved block trade position limits

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Example:

`"abcdef12-3456-7890-abcd-ef1234567890"`