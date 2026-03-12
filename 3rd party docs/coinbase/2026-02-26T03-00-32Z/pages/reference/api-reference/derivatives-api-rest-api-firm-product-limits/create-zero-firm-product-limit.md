# create zero firm product limit

Create Zero Firm Product Limit

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/firm-product-limit \
  --header 'Content-Type: application/json' \
  --data '
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_code": "BIPZ30"
}
'
```

```
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_code": "BIPZ30",
  "product_name": "Bitcoin Perpetual Index Futures Dec 2030",
  "trading_disabled": false
}
```

Creates a limit that will block the firm from trading the corresponding product.

Create Zero Firm Product Limit

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/firm-product-limit \
  --header 'Content-Type: application/json' \
  --data '
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_code": "BIPZ30"
}
'
```

```
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_code": "BIPZ30",
  "product_name": "Bitcoin Perpetual Index Futures Dec 2030",
  "trading_disabled": false
}
```

#### Body

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

#### Response

Successfully created firm product limit

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

Example:

`"Bitcoin Perpetual Index Futures Dec 2030"`