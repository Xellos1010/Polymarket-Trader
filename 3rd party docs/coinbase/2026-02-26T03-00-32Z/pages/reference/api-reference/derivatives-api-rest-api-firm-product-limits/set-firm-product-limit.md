# set firm product limit

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/v2/firm-product-limit \
  --header 'Content-Type: application/json' \
  --data '
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_code": "BIPZ30",
  "trading_disabled": false,
  "trading24x7_disabled": true,
  "options_fill_protection_threshold": 100,
  "long_daily_position_limit": 10000,
  "short_daily_position_limit": 15000,
  "long_real_position_limit": 15000,
  "short_real_position_limit": 20000
}
'
```

```
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_code": "BIPZ30",
  "product_name": "Bitcoin Perpetual Index Futures Dec 2030",
  "trading_disabled": false,
  "trading24x7_disabled": true,
  "long_daily_position_limit": 10000,
  "short_daily_position_limit": 15000,
  "long_real_position_limit": 15000,
  "short_real_position_limit": 20000
}
```

Set position limits on a product.

POST

/

rest

/

v2

/

firm-product-limit

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/v2/firm-product-limit \
  --header 'Content-Type: application/json' \
  --data '
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_code": "BIPZ30",
  "trading_disabled": false,
  "trading24x7_disabled": true,
  "options_fill_protection_threshold": 100,
  "long_daily_position_limit": 10000,
  "short_daily_position_limit": 15000,
  "long_real_position_limit": 15000,
  "short_real_position_limit": 20000
}
'
```

```
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_code": "BIPZ30",
  "product_name": "Bitcoin Perpetual Index Futures Dec 2030",
  "trading_disabled": false,
  "trading24x7_disabled": true,
  "long_daily_position_limit": 10000,
  "short_daily_position_limit": 15000,
  "long_real_position_limit": 15000,
  "short_real_position_limit": 20000
}
```

#### Body

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

options\_fill\_protection\_threshold

long\_daily\_position\_limit

short\_daily\_position\_limit

short\_real\_position\_limit

#### Response

Successfully created firm product limit

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

Example:

`"Bitcoin Perpetual Index Futures Dec 2030"`

long\_daily\_position\_limit

short\_daily\_position\_limit

short\_real\_position\_limit