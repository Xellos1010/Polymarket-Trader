# kill switch

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/kill-switch \
  --header 'Content-Type: application/json' \
  --data '
{
  "only_cancel_no_trading_lock": true,
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

Activates a kill switch to cancel orders and lock trading for specified users, firms, or FCMs. Must only include one uuid (fcm, firm, or trading user). When activated, all open orders will be cancelled and new orders will be rejected.

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/kill-switch \
  --header 'Content-Type: application/json' \
  --data '
{
  "only_cancel_no_trading_lock": true,
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

only\_cancel\_no\_trading\_lock

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Example:

`"aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"`

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

#### Response

Kill switch executed successfully