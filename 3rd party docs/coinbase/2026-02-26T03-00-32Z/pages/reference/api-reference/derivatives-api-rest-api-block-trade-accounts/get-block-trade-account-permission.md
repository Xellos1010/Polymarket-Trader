# get block trade account permission

Get Block Trade Account Permission

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade-account-permissions/{uuid}
```

```
{
  "permission_uuid": "123e4567-e89b-12d3-a456-426614174000",
  "block_trade_entity_uuid": "abcdef12-3456-7890-abcd-ef1234567890",
  "account_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6"
}
```

Retrieves information for a block trade account permission with the specified UUID.

GET

/

rest

/

block-trade-account-permissions

/

{uuid}

Get Block Trade Account Permission

```
curl --request GET \
  --url https://api.exchange.fairx.net/rest/block-trade-account-permissions/{uuid}
```

```
{
  "permission_uuid": "123e4567-e89b-12d3-a456-426614174000",
  "block_trade_entity_uuid": "abcdef12-3456-7890-abcd-ef1234567890",
  "account_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6"
}
```

#### Path Parameters

The block-trade-account-permission's UUID

Example:

`"e80d6a4e-af9f-4fcb-a819-3d22c7017279"`

#### Response

Successfully retrieved block trade account permission

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Example:

`"abcdef12-3456-7890-abcd-ef1234567890"`

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`