# remove firm product limit

Remove Firm Product Limit

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/firm-product-limit/remove \
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

Removes any product level limit for a specific firm and product

POST

/

rest

/

firm-product-limit

/

remove

Remove Firm Product Limit

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/firm-product-limit/remove \
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

Product limit removal parameters

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

#### Response

Successfully removed firm product limit

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

Example:

`"Bitcoin Perpetual Index Futures Dec 2030"`