# unlock trading

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/unlock-trading \
  --header 'Content-Type: application/json' \
  --data '
{
  "fcm_uuid": "123e4567-e89b-12d3-a456-426614174000",
  "firm_uuid": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
  "trading_user_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6"
}
'
```

```
{
  "num_users_affected": 4,
  "num_orders_canceled": 0
}
```

Deactivates kill switch to cancel orders and unlocks trading for specified users, firms, or FCMs. Must only include one uuid (fcm, firm, or trading user).

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/unlock-trading \
  --header 'Content-Type: application/json' \
  --data '
{
  "fcm_uuid": "123e4567-e89b-12d3-a456-426614174000",
  "firm_uuid": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
  "trading_user_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6"
}
'
```

```
{
  "num_users_affected": 4,
  "num_orders_canceled": 0
}
```

#### Body

Unlock Trading parameters

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Example:

`"aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"`

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

#### Response

Unlock trading executed successfully. Note: numOrdersCanceled will always be 0